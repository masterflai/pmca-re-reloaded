//! Runtime assembly of the updater-shell firmware image (`FDAT`).
//!
//! The program ships no Sony firmware files. Under `updatershell/bodyfs/` it
//! carries the unencrypted updater body of each chip family (`<chip>.fdat`, the
//! open OpenMemories body plus a numeric header) and a `models.tsv` table
//! mapping each camera model to its chip and its numeric model/region codes.
//!
//! At run time the right template is retargeted at the connected camera and
//! encrypted with the FDAT key the user imported into [`keystore`]; the key
//! lives only in the session.

use std::path::Path;

use pmca_core::fdat::{self, Crypter};
use pmca_core::keystore::{self, KeyId};
use pmca_i18n::Localizer;

use crate::{Error, Result};

/// One row of the shipped model table.
struct ModelInfo {
    /// Chip family, selecting the body template and the crypter.
    chip: String,
    /// Numeric model identifier the camera checks against itself.
    model: u32,
    /// Numeric region identifier.
    region: u32,
}

/// Parses `bodyfs/models.tsv` into `(model name, info)` rows.
fn load_table(base: &Path) -> Result<Vec<(String, ModelInfo)>> {
    let text = std::fs::read_to_string(base.join("models.tsv"))?;
    let mut out = Vec::new();
    for line in text.lines() {
        let fields: Vec<&str> = line.split('\t').collect();
        if let [name, chip, model, region] = fields[..] {
            let (Ok(model), Ok(region)) =
                (u32::from_str_radix(model, 16), u32::from_str_radix(region, 16))
            else {
                continue;
            };
            out.push((name.to_string(), ModelInfo { chip: chip.to_string(), model, region }));
        }
    }
    Ok(out)
}

/// Finds a model, trimming trailing suffix characters (`ILCE-5100L` ->
/// `ILCE-5100`) exactly as the reference firmware selection does.
fn find<'a>(table: &'a [(String, ModelInfo)], model: &str) -> Option<&'a ModelInfo> {
    let mut candidate = model;
    loop {
        if let Some((_, info)) = table.iter().find(|(name, _)| name == candidate) {
            return Some(info);
        }
        match candidate.chars().last() {
            None => return None,
            Some(c) if c.is_ascii_digit() => return None,
            Some(_) => candidate = &candidate[..candidate.len() - 1],
        }
    }
}

/// Reads an FDAT key from the store, or reports it as missing to the front-end.
fn fdat_key(l: &Localizer, id: KeyId) -> Result<Vec<u8>> {
    keystore::get(id).ok_or_else(|| Error::MissingKeys(l.tr("keys-not-set-up")))
}

/// Builds the encrypter for a chip family from the imported FDAT keys.
fn crypter_for(l: &Localizer, chip: &str) -> Result<Crypter> {
    let crypter = match chip {
        "CXD4105" => Crypter::sha(&fdat_key(l, KeyId::FdatCxd4105)?),
        "CXD4115" => Crypter::sha(&fdat_key(l, KeyId::FdatCxd4115)?),
        "CXD4115_ilc" => Crypter::sha(&fdat_key(l, KeyId::FdatCxd4115Ilc)?),
        "CXD4120" => Crypter::sha(&fdat_key(l, KeyId::FdatCxd4120)?),
        "CXD4132" => Crypter::aes(&fdat_key(l, KeyId::FdatCxd4132)?),
        "CXD90014" => Crypter::double_aes(
            &fdat_key(l, KeyId::FdatCxd4132)?,
            &fdat_key(l, KeyId::FdatCxd90014)?,
        ),
        other => return Err(Error::Operation(format!("unknown updater chip {other}"))),
    };
    crypter.map_err(|e| Error::Operation(e.to_string()))
}

/// Generates the encrypted updater-shell FDAT image for `model`.
///
/// `base` is the `updatershell/bodyfs` directory. Fails with
/// [`Error::MissingKeys`] when the FDAT key for the model's chip has not been
/// imported, so a front-end can send the user to the key import.
pub fn generate(l: &Localizer, base: &Path, model: &str) -> Result<Vec<u8>> {
    let table = load_table(base)?;
    let info = find(&table, model)
        .ok_or_else(|| Error::Operation(l.tr_arg("updater-no-fdat", "model", model)))?;

    let mut image = std::fs::read(base.join(format!("{}.fdat", info.chip)))?;
    let header =
        fdat::read_header(&image).map_err(|e| Error::Operation(format!("{}: {e}", info.chip)))?;
    fdat::retarget(&mut image, info.model, info.region, header.version_major, header.version_minor)
        .map_err(|e| Error::Operation(e.to_string()))?;

    crypter_for(l, &info.chip)?.encrypt(&image).map_err(|e| Error::Operation(e.to_string()))
}

/// Returns every FDAT crypter whose key material is currently in the store.
///
/// Used to identify the chip key of an arbitrary image by trial decryption,
/// without having to detect the camera model first.
fn available_crypters() -> Vec<Crypter> {
    let mut out = Vec::new();
    for id in [KeyId::FdatCxd4105, KeyId::FdatCxd4115, KeyId::FdatCxd4115Ilc, KeyId::FdatCxd4120] {
        if let Some(key) = keystore::get(id)
            && let Ok(crypter) = Crypter::sha(&key)
        {
            out.push(crypter);
        }
    }
    if let Some(key) = keystore::get(KeyId::FdatCxd4132) {
        if let Ok(crypter) = Crypter::aes(&key) {
            out.push(crypter);
        }
        if let Some(inner) = keystore::get(KeyId::FdatCxd90014)
            && let Ok(crypter) = Crypter::double_aes(&key, &inner)
        {
            out.push(crypter);
        }
    }
    out
}

/// An image retargeted for re-flashing, plus the version it really carries.
pub struct ReflashImage {
    /// The re-encrypted FDAT with its header version raised past the installed one.
    pub payload: Vec<u8>,
    /// The image's genuine major version (before the version was raised).
    pub real_version_major: u8,
    /// The image's genuine minor version.
    pub real_version_minor: u8,
}

/// Rewrites an encrypted `FDAT` so its header declares `target_major.target_minor`,
/// letting the camera accept it when that version beats the installed one.
///
/// Model, region and the firmware body are left untouched. The camera *reports*
/// the header version after a flash, so the genuine version is returned here to
/// be restored afterwards (see [`crate::firmware`]). The caller passes the
/// smallest version that clears the gate (installed + 1), so an interrupted
/// restore leaves the camera one step above its real version rather than at an
/// unreachable maximum. The right chip key is found by trying each imported FDAT
/// key and keeping the one that decrypts to a valid header.
pub fn force_version(
    l: &Localizer,
    encrypted_fdat: &[u8],
    target_major: u8,
    target_minor: u8,
) -> Result<ReflashImage> {
    let crypters = available_crypters();
    if crypters.is_empty() {
        return Err(Error::MissingKeys(l.tr("keys-not-set-up")));
    }
    for mut crypter in crypters {
        let Ok(mut image) = crypter.decrypt(encrypted_fdat) else { continue };
        let Ok(header) = fdat::read_header(&image) else { continue };
        let (real_version_major, real_version_minor) = (header.version_major, header.version_minor);
        fdat::retarget(&mut image, header.model, header.region, target_major, target_minor)
            .map_err(|e| Error::Operation(e.to_string()))?;
        let payload = crypter.encrypt(&image).map_err(|e| Error::Operation(e.to_string()))?;
        return Ok(ReflashImage { payload, real_version_major, real_version_minor });
    }
    Err(Error::MissingKeys(l.tr("keys-not-set-up")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn force_version_raises_header_but_keeps_model_and_body() {
        let _g = keystore::test_guard();
        keystore::install_test_keys();

        // A decrypted FDAT declaring version 3.21 for a made-up model/region.
        let model = 0x0040_0000;
        let region = 0x0002_0000;
        let plain = fdat::assemble(&[0u8; 32], &[0xab; 48], model, region, 3, 0x21);
        let encrypted = Crypter::aes(&[0x66; 16]).unwrap().encrypt(&plain).unwrap();

        let out = force_version(&Localizer::new("en-US"), &encrypted, 3, 0x22).unwrap();
        assert_eq!((out.real_version_major, out.real_version_minor), (3, 0x21));

        let restored = Crypter::aes(&[0x66; 16]).unwrap().decrypt(&out.payload).unwrap();
        let header = fdat::read_header(&restored).unwrap();
        assert_eq!((header.version_major, header.version_minor), (3, 0x22));
        assert_eq!(header.model, model);
        assert_eq!(header.region, region);
        assert_eq!(&restored[header.firmware_offset as usize..], &[0xab; 48]);
    }
}
