//! The shutter-count report as a file: a single page in PDF around a JPEG.
//!
//! The window draws the page; this only wraps the picture, so no PDF library
//! and no font handling are needed.

/// Paper the page is laid out for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Paper {
    /// ISO A4, 210 × 297 mm.
    A4,
    /// US Letter, 8.5 × 11 in.
    Letter,
}

impl Paper {
    /// The paper named `a4` or `letter`.
    #[must_use]
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "a4" => Some(Self::A4),
            "letter" => Some(Self::Letter),
            _ => None,
        }
    }

    /// Width and height in PDF points (1/72 inch).
    #[must_use]
    pub fn points(self) -> (u16, u16) {
        match self {
            Self::A4 => (595, 842),
            Self::Letter => (612, 792),
        }
    }
}

/// Width, height and number of colour components of a JPEG.
#[must_use]
pub fn jpeg_size(data: &[u8]) -> Option<(u16, u16, u8)> {
    if !data.starts_with(&[0xFF, 0xD8]) {
        return None;
    }
    let mut at = 2;
    while at + 4 <= data.len() {
        if data[at] != 0xFF {
            return None;
        }
        let marker = data[at + 1];
        if marker == 0xFF {
            at += 1;
            continue;
        }
        if marker == 0x01 || (0xD0..=0xD7).contains(&marker) {
            at += 2;
            continue;
        }
        if marker == 0xD9 || marker == 0xDA {
            return None;
        }
        let len = usize::from(u16::from_be_bytes([data[at + 2], data[at + 3]]));
        if len < 2 {
            return None;
        }
        // Start of frame, any coding except the DHT/JPG/DAC markers in that range.
        if matches!(marker, 0xC0..=0xC3 | 0xC5..=0xC7 | 0xC9..=0xCB | 0xCD..=0xCF) {
            let frame = data.get(at + 4..at + 2 + len)?;
            let (&[_, h1, h0, w1, w0, components], _) = frame.split_first_chunk::<6>()?;
            let (width, height) = (u16::from_be_bytes([w1, w0]), u16::from_be_bytes([h1, h0]));
            return (width > 0 && height > 0).then_some((width, height, components));
        }
        at += 2 + len;
    }
    None
}

/// A one-page PDF with `jpeg` scaled to fit the page; `None` if it is not a
/// greyscale or RGB JPEG.
#[must_use]
pub fn pdf_from_jpeg(jpeg: &[u8], paper: Paper) -> Option<Vec<u8>> {
    let (width, height, components) = jpeg_size(jpeg)?;
    let colour = match components {
        1 => "DeviceGray",
        3 => "DeviceRGB",
        _ => return None,
    };
    let (page_w, page_h) = paper.points();
    let (page_w, page_h) = (f64::from(page_w), f64::from(page_h));
    let scale = f64::min(page_w / f64::from(width), page_h / f64::from(height));
    let (draw_w, draw_h) = (f64::from(width) * scale, f64::from(height) * scale);
    let (x, y) = ((page_w - draw_w) / 2.0, (page_h - draw_h) / 2.0);
    let content = format!("q {draw_w:.2} 0 0 {draw_h:.2} {x:.2} {y:.2} cm /Im0 Do Q\n");

    let mut out = b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n".to_vec();
    let mut offsets = Vec::with_capacity(5);
    let mut object = |out: &mut Vec<u8>, head: String, stream: Option<&[u8]>| {
        offsets.push(out.len());
        out.extend(format!("{} 0 obj\n{head}\n", offsets.len()).as_bytes());
        if let Some(bytes) = stream {
            out.extend(b"stream\n");
            out.extend(bytes);
            out.extend(b"\nendstream\n");
        }
        out.extend(b"endobj\n");
    };
    object(&mut out, "<< /Type /Catalog /Pages 2 0 R >>".into(), None);
    object(&mut out, "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".into(), None);
    object(
        &mut out,
        format!(
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {page_w} {page_h}] \
             /Resources << /XObject << /Im0 4 0 R >> >> /Contents 5 0 R >>"
        ),
        None,
    );
    object(
        &mut out,
        format!(
            "<< /Type /XObject /Subtype /Image /Width {width} /Height {height} \
             /ColorSpace /{colour} /BitsPerComponent 8 /Filter /DCTDecode /Length {} >>",
            jpeg.len()
        ),
        Some(jpeg),
    );
    object(&mut out, format!("<< /Length {} >>", content.len()), Some(content.as_bytes()));

    let xref = out.len();
    out.extend(format!("xref\n0 {}\n0000000000 65535 f \n", offsets.len() + 1).as_bytes());
    for offset in &offsets {
        out.extend(format!("{offset:010} 00000 n \n").as_bytes());
    }
    out.extend(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n",
            offsets.len() + 1
        )
        .as_bytes(),
    );
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// SOI, an APP0 segment to skip, then a baseline frame header.
    fn jpeg(width: u16, height: u16, components: u8) -> Vec<u8> {
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x04, 0x4A, 0x46];
        data.extend([0xFF, 0xC0, 0x00, 0x0B, 0x08]);
        data.extend(height.to_be_bytes());
        data.extend(width.to_be_bytes());
        data.extend([components, 0x01, 0x11, 0x00]);
        data.extend([0xFF, 0xD9]);
        data
    }

    #[test]
    fn reads_the_frame_size() {
        assert_eq!(jpeg_size(&jpeg(1240, 1754, 3)), Some((1240, 1754, 3)));
        assert_eq!(jpeg_size(b"\x89PNG\r\n\x1a\n"), None);
        assert_eq!(jpeg_size(&[0xFF, 0xD8, 0xFF, 0xD9]), None);
    }

    #[test]
    fn the_cross_reference_points_at_every_object() {
        let image = jpeg(1275, 1650, 3);
        let pdf = pdf_from_jpeg(&image, Paper::Letter).unwrap();
        assert!(pdf.starts_with(b"%PDF-1.4"));
        // Everything from the image stream on is ASCII; the binary comment is before it.
        let tail_at = pdf.windows(9).rposition(|w| w == b"endstream").unwrap();
        let tail = std::str::from_utf8(&pdf[tail_at..]).unwrap();
        assert!(tail.ends_with("%%EOF\n"));
        let xref: usize =
            tail.rsplit("startxref\n").next().unwrap().lines().next().unwrap().parse().unwrap();
        let table = std::str::from_utf8(&pdf[xref..]).unwrap();
        assert!(table.starts_with("xref\n0 6\n"));
        let entries: Vec<usize> =
            table.lines().skip(3).take(5).map(|line| line[..10].parse().unwrap()).collect();
        for (number, offset) in entries.iter().enumerate() {
            assert!(pdf[*offset..].starts_with(format!("{} 0 obj", number + 1).as_bytes()));
        }
        let page = std::str::from_utf8(&pdf[entries[2]..entries[3]]).unwrap();
        assert!(page.contains("/MediaBox [0 0 612 792]"));
    }

    #[test]
    fn only_grey_and_rgb_images_are_wrapped() {
        assert!(pdf_from_jpeg(&jpeg(10, 10, 4), Paper::A4).is_none());
        assert!(pdf_from_jpeg(&jpeg(10, 10, 1), Paper::A4).is_some());
    }
}
