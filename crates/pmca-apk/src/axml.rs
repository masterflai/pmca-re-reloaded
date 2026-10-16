//! Decoder for Android's *binary* XML format (`AndroidManifest.xml` inside an
//! APK).
//!
//! Only the parts required to describe an application are decoded: the string
//! pool and the start-element chunks with their attributes. Namespaces,
//! styles, CDATA and the resource map are skipped.
//!
//! The original Python used the `axmlparserpy` package, which indexes into the
//! buffer without validation. Every read here is bounds-checked and returns
//! [`Error::Axml`] instead of panicking, because an APK is untrusted input.

use crate::{Error, Result};

/// `RES_XML_TYPE` — the outer chunk of a binary XML document.
const RES_XML_TYPE: u16 = 0x0003;
/// `RES_STRING_POOL_TYPE`.
const RES_STRING_POOL_TYPE: u16 = 0x0001;
/// `RES_XML_START_ELEMENT_TYPE`.
const RES_XML_START_ELEMENT_TYPE: u16 = 0x0102;

/// String-pool flag marking the entries as (modified) UTF-8 rather than UTF-16.
const UTF8_FLAG: u32 = 1 << 8;

/// `Res_value::TYPE_STRING`.
const TYPE_STRING: u8 = 0x03;
/// `Res_value::TYPE_INT_DEC`.
const TYPE_INT_DEC: u8 = 0x10;
/// `Res_value::TYPE_INT_HEX`.
const TYPE_INT_HEX: u8 = 0x11;
/// `Res_value::TYPE_INT_BOOLEAN`.
const TYPE_INT_BOOLEAN: u8 = 0x12;

/// Sentinel used by Android for "no string".
const NO_STRING: u32 = 0xffff_ffff;

/// Smallest attribute record Android emits (`ResXMLTree_attribute`).
const ATTRIBUTE_SIZE: usize = 20;

/// A decoded attribute value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// A string from the document's string pool.
    Str(String),
    /// An integer (decimal or hexadecimal in the source).
    Int(u32),
    /// A boolean.
    Bool(bool),
    /// A value of a type this decoder does not interpret; the raw 32-bit
    /// payload is preserved.
    Raw(u32),
}

impl Value {
    /// The value as a string, if it is one.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }

    /// The value as an integer, if it is one.
    pub fn as_int(&self) -> Option<u32> {
        match self {
            Value::Int(i) => Some(*i),
            Value::Bool(b) => Some(u32::from(*b)),
            _ => None,
        }
    }
}

/// A single attribute of a start element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    /// Local attribute name without the namespace prefix (e.g. `versionCode`).
    pub name: String,
    /// Decoded value.
    pub value: Value,
}

/// A start element, in document order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    /// Local element name (e.g. `manifest`).
    pub name: String,
    /// The element's attributes.
    pub attributes: Vec<Attribute>,
}

impl Element {
    /// Look up an attribute by its local name.
    pub fn attr(&self, name: &str) -> Option<&Value> {
        self.attributes.iter().find(|a| a.name == name).map(|a| &a.value)
    }
}

/// Decode a binary `AndroidManifest.xml` into its start elements.
pub fn parse(data: &[u8]) -> Result<Vec<Element>> {
    if u16le(data, 0)? != RES_XML_TYPE {
        return Err(Error::Axml("not a binary xml document"));
    }
    let declared = u32le(data, 4)? as usize;
    // A truncated file must not make us read past the buffer, and an inflated
    // `size` field must not let us read past it either.
    let end = declared.min(data.len());

    let mut pos = u16le(data, 2)? as usize;
    let mut pool: Option<Vec<String>> = None;
    let mut elements = Vec::new();

    while pos.saturating_add(8) <= end {
        let chunk_type = u16le(data, pos)?;
        let chunk_size = u32le(data, pos + 4)? as usize;
        if chunk_size < 8 {
            return Err(Error::Axml("chunk size too small"));
        }
        let chunk_end = pos.checked_add(chunk_size).ok_or(Error::Axml("chunk size overflow"))?;
        if chunk_end > end {
            return Err(Error::Axml("chunk exceeds document"));
        }
        let chunk = &data[pos..chunk_end];

        match chunk_type {
            RES_STRING_POOL_TYPE => pool = Some(parse_string_pool(chunk)?),
            RES_XML_START_ELEMENT_TYPE => {
                let pool =
                    pool.as_deref().ok_or(Error::Axml("start element before string pool"))?;
                elements.push(parse_start_element(chunk, pool)?);
            }
            _ => {}
        }
        pos = chunk_end;
    }
    Ok(elements)
}

/// Decode the document's string pool.
fn parse_string_pool(chunk: &[u8]) -> Result<Vec<String>> {
    let header_size = u16le(chunk, 2)? as usize;
    let count = u32le(chunk, 8)? as usize;
    let flags = u32le(chunk, 16)?;
    let strings_start = u32le(chunk, 20)? as usize;
    let utf8 = flags & UTF8_FLAG != 0;

    // Reject a bogus count before allocating: every entry needs a 4-byte offset.
    let offsets_end =
        count.checked_mul(4).and_then(|n| n.checked_add(header_size)).unwrap_or(usize::MAX);
    if offsets_end > chunk.len() {
        return Err(Error::Axml("string pool offset table out of bounds"));
    }

    let mut strings = Vec::with_capacity(count);
    for i in 0..count {
        let offset = u32le(chunk, header_size + i * 4)? as usize;
        let pos = strings_start.checked_add(offset).ok_or(Error::Axml("string offset overflow"))?;
        strings.push(if utf8 { read_utf8(chunk, pos)? } else { read_utf16(chunk, pos)? });
    }
    Ok(strings)
}

/// Read a length-prefixed UTF-16 pool entry.
fn read_utf16(chunk: &[u8], pos: usize) -> Result<String> {
    let (len, mut pos) = {
        let first = u16le(chunk, pos)? as usize;
        if first & 0x8000 != 0 {
            let low = u16le(chunk, pos + 2)? as usize;
            (((first & 0x7fff) << 16) | low, pos + 4)
        } else {
            (first, pos + 2)
        }
    };
    let mut units = Vec::with_capacity(len.min(chunk.len() / 2));
    for _ in 0..len {
        units.push(u16le(chunk, pos)?);
        pos += 2;
    }
    Ok(String::from_utf16_lossy(&units))
}

/// Read a length-prefixed (modified) UTF-8 pool entry.
///
/// Two varints precede the data: the UTF-16 length and the byte length. Only
/// the byte length is needed.
fn read_utf8(chunk: &[u8], pos: usize) -> Result<String> {
    let (_utf16_len, pos) = read_varint8(chunk, pos)?;
    let (byte_len, pos) = read_varint8(chunk, pos)?;
    let end = pos.checked_add(byte_len).ok_or(Error::Axml("string length overflow"))?;
    let bytes = chunk.get(pos..end).ok_or(Error::Axml("string out of bounds"))?;
    Ok(String::from_utf8_lossy(bytes).into_owned())
}

/// Read Android's 1-or-2 byte length prefix, returning `(value, next_offset)`.
fn read_varint8(chunk: &[u8], pos: usize) -> Result<(usize, usize)> {
    let first = *chunk.get(pos).ok_or(Error::Axml("truncated string length"))? as usize;
    if first & 0x80 != 0 {
        let second = *chunk.get(pos + 1).ok_or(Error::Axml("truncated string length"))? as usize;
        Ok((((first & 0x7f) << 8) | second, pos + 2))
    } else {
        Ok((first, pos + 1))
    }
}

/// Decode one `RES_XML_START_ELEMENT_TYPE` chunk.
fn parse_start_element(chunk: &[u8], pool: &[String]) -> Result<Element> {
    let base = u16le(chunk, 2)? as usize;
    let name = string_at(pool, u32le(chunk, base + 4)?)
        .ok_or(Error::Axml("element name not in string pool"))?
        .to_string();
    let attr_start = u16le(chunk, base + 8)? as usize;
    let attr_size = u16le(chunk, base + 10)? as usize;
    let attr_count = u16le(chunk, base + 12)? as usize;

    if attr_size < ATTRIBUTE_SIZE {
        return Err(Error::Axml("attribute record too small"));
    }
    let first = base.checked_add(attr_start).ok_or(Error::Axml("attribute offset overflow"))?;
    let span = attr_count.checked_mul(attr_size).ok_or(Error::Axml("attribute span overflow"))?;
    if first.checked_add(span).ok_or(Error::Axml("attribute span overflow"))? > chunk.len() {
        return Err(Error::Axml("attributes exceed chunk"));
    }

    let mut attributes = Vec::with_capacity(attr_count);
    for i in 0..attr_count {
        let at = first + i * attr_size;
        let Some(name) = string_at(pool, u32le(chunk, at + 4)?) else {
            // An attribute we cannot name is useless but not fatal.
            continue;
        };
        let raw = u32le(chunk, at + 8)?;
        let data_type = *chunk.get(at + 15).ok_or(Error::Axml("truncated attribute"))?;
        let data = u32le(chunk, at + 16)?;
        let value = match data_type {
            TYPE_STRING => Value::Str(
                string_at(pool, data)
                    .or_else(|| string_at(pool, raw))
                    .unwrap_or_default()
                    .to_string(),
            ),
            TYPE_INT_DEC | TYPE_INT_HEX => Value::Int(data),
            TYPE_INT_BOOLEAN => Value::Bool(data != 0),
            _ => Value::Raw(data),
        };
        attributes.push(Attribute { name: name.to_string(), value });
    }
    Ok(Element { name, attributes })
}

/// Resolve a string-pool index, treating [`NO_STRING`] and out-of-range indices
/// as absent.
fn string_at(pool: &[String], index: u32) -> Option<&str> {
    if index == NO_STRING {
        return None;
    }
    pool.get(index as usize).map(String::as_str)
}

/// Bounds-checked little-endian `u16`.
fn u16le(data: &[u8], off: usize) -> Result<u16> {
    let end = off.checked_add(2).ok_or(Error::Axml("offset overflow"))?;
    let b = data.get(off..end).ok_or(Error::Axml("truncated u16"))?;
    Ok(u16::from_le_bytes([b[0], b[1]]))
}

/// Bounds-checked little-endian `u32`.
fn u32le(data: &[u8], off: usize) -> Result<u32> {
    let end = off.checked_add(4).ok_or(Error::Axml("offset overflow"))?;
    let b = data.get(off..end).ok_or(Error::Axml("truncated u32"))?;
    Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Build a minimal but structurally valid binary XML document containing a
    /// `<manifest>` and a `<uses-sdk>` element.
    pub(crate) fn sample(utf8: bool) -> Vec<u8> {
        let strings = [
            "manifest",
            "package",
            "versionCode",
            "versionName",
            "uses-sdk",
            "minSdkVersion",
            "com.example.app",
            "1.2.3",
        ];

        // --- string pool ---
        let mut data = Vec::new();
        let mut offsets = Vec::new();
        for s in strings {
            offsets.push(data.len() as u32);
            if utf8 {
                data.push(s.chars().count() as u8);
                data.push(s.len() as u8);
                data.extend_from_slice(s.as_bytes());
                data.push(0);
            } else {
                data.extend_from_slice(&(s.encode_utf16().count() as u16).to_le_bytes());
                for u in s.encode_utf16() {
                    data.extend_from_slice(&u.to_le_bytes());
                }
                data.extend_from_slice(&0u16.to_le_bytes());
            }
        }
        while data.len() % 4 != 0 {
            data.push(0);
        }

        let header_size = 28usize;
        let strings_start = header_size + offsets.len() * 4;
        let pool_size = strings_start + data.len();
        let mut pool = Vec::new();
        pool.extend_from_slice(&RES_STRING_POOL_TYPE.to_le_bytes());
        pool.extend_from_slice(&(header_size as u16).to_le_bytes());
        pool.extend_from_slice(&(pool_size as u32).to_le_bytes());
        pool.extend_from_slice(&(offsets.len() as u32).to_le_bytes());
        pool.extend_from_slice(&0u32.to_le_bytes()); // styleCount
        pool.extend_from_slice(&(if utf8 { UTF8_FLAG } else { 0 }).to_le_bytes());
        pool.extend_from_slice(&(strings_start as u32).to_le_bytes());
        pool.extend_from_slice(&0u32.to_le_bytes()); // stylesStart
        for o in &offsets {
            pool.extend_from_slice(&o.to_le_bytes());
        }
        pool.extend_from_slice(&data);

        // --- start elements ---
        let element = |name: u32, attrs: &[(u32, u8, u32)]| -> Vec<u8> {
            let size = 16 + 20 + attrs.len() * ATTRIBUTE_SIZE;
            let mut c = Vec::new();
            c.extend_from_slice(&RES_XML_START_ELEMENT_TYPE.to_le_bytes());
            c.extend_from_slice(&16u16.to_le_bytes()); // headerSize
            c.extend_from_slice(&(size as u32).to_le_bytes());
            c.extend_from_slice(&1u32.to_le_bytes()); // lineNumber
            c.extend_from_slice(&NO_STRING.to_le_bytes()); // comment
            c.extend_from_slice(&NO_STRING.to_le_bytes()); // ns
            c.extend_from_slice(&name.to_le_bytes());
            c.extend_from_slice(&20u16.to_le_bytes()); // attributeStart
            c.extend_from_slice(&(ATTRIBUTE_SIZE as u16).to_le_bytes());
            c.extend_from_slice(&(attrs.len() as u16).to_le_bytes());
            c.extend_from_slice(&0u16.to_le_bytes()); // idIndex
            c.extend_from_slice(&0u16.to_le_bytes()); // classIndex
            c.extend_from_slice(&0u16.to_le_bytes()); // styleIndex
            for (name, ty, value) in attrs {
                c.extend_from_slice(&NO_STRING.to_le_bytes()); // ns
                c.extend_from_slice(&name.to_le_bytes());
                c.extend_from_slice(&NO_STRING.to_le_bytes()); // rawValue
                c.extend_from_slice(&8u16.to_le_bytes()); // size
                c.push(0); // res0
                c.push(*ty);
                c.extend_from_slice(&value.to_le_bytes());
            }
            c
        };

        let manifest =
            element(0, &[(1, TYPE_STRING, 6), (2, TYPE_INT_DEC, 42), (3, TYPE_STRING, 7)]);
        let uses_sdk = element(4, &[(5, TYPE_INT_DEC, 10)]);

        let total = 8 + pool.len() + manifest.len() + uses_sdk.len();
        let mut out = Vec::with_capacity(total);
        out.extend_from_slice(&RES_XML_TYPE.to_le_bytes());
        out.extend_from_slice(&8u16.to_le_bytes());
        out.extend_from_slice(&(total as u32).to_le_bytes());
        out.extend_from_slice(&pool);
        out.extend_from_slice(&manifest);
        out.extend_from_slice(&uses_sdk);
        out
    }

    #[test]
    fn decodes_utf16_documents() {
        let elements = parse(&sample(false)).unwrap();
        assert_eq!(elements.len(), 2);
        assert_eq!(elements[0].name, "manifest");
        assert_eq!(elements[0].attr("package").unwrap().as_str(), Some("com.example.app"));
        assert_eq!(elements[0].attr("versionCode").unwrap().as_int(), Some(42));
        assert_eq!(elements[0].attr("versionName").unwrap().as_str(), Some("1.2.3"));
        assert_eq!(elements[1].name, "uses-sdk");
        assert_eq!(elements[1].attr("minSdkVersion").unwrap().as_int(), Some(10));
    }

    #[test]
    fn decodes_utf8_documents() {
        let elements = parse(&sample(true)).unwrap();
        assert_eq!(elements[0].attr("package").unwrap().as_str(), Some("com.example.app"));
        assert_eq!(elements[1].attr("minSdkVersion").unwrap().as_int(), Some(10));
    }

    #[test]
    fn rejects_a_foreign_magic() {
        assert!(parse(&[0, 0, 8, 0, 8, 0, 0, 0]).is_err());
    }

    #[test]
    fn rejects_an_empty_buffer() {
        assert!(parse(&[]).is_err());
    }

    #[test]
    fn truncation_is_an_error_not_a_panic() {
        let full = sample(false);
        for cut in 1..full.len() {
            // Must never panic, whatever the truncation point.
            let _ = parse(&full[..cut]);
        }
    }

    #[test]
    fn a_zero_sized_chunk_is_rejected() {
        let mut doc = sample(false);
        // Blank out the string-pool chunk size.
        doc[12] = 0;
        doc[13] = 0;
        doc[14] = 0;
        doc[15] = 0;
        assert!(parse(&doc).is_err());
    }

    #[test]
    fn missing_attributes_read_as_none() {
        let elements = parse(&sample(false)).unwrap();
        assert!(elements[0].attr("nope").is_none());
    }

    #[test]
    fn boolean_values_convert_to_ints() {
        assert_eq!(Value::Bool(true).as_int(), Some(1));
        assert_eq!(Value::Bool(false).as_int(), Some(0));
        assert_eq!(Value::Raw(7).as_int(), None);
        assert_eq!(Value::Raw(7).as_str(), None);
    }
}
