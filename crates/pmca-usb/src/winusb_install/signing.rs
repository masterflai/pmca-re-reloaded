//! Throw-away code-signing: certificate, catalog, signature.
//!
//! Windows refuses to stage a third-party driver package whose INF is not
//! covered by a signed catalog — measured, with no override and no "install
//! anyway" prompt (`SPAPI_E_NO_CATALOG_FOR_OEM_INF`). This module produces
//! that catalog.
//!
//! The certificate it creates is deliberately short-lived. Trust for a driver
//! package is evaluated **once**, while the package is being copied into the
//! DriverStore; afterwards the store itself is the trust anchor. So the caller
//! stages the package and then drops the certificate immediately, and the
//! binding keeps working — including across reboots and re-plugs. That is what
//! keeps the security cost of this approach bounded: a self-signed certificate
//! exists in the machine's trust stores for the duration of one file copy, not
//! for the lifetime of the installation.
//!
//! The private key is an unnamed NCrypt key: it exists only in this process's
//! memory, is never persisted, and goes away with the certificate context.

use std::ffi::c_void;
use std::path::Path;

use windows::Win32::Foundation::{CloseHandle, E_ACCESSDENIED, FILETIME, HANDLE, SYSTEMTIME};
use windows::Win32::Security::Cryptography::Catalog::{
    CRYPTCAT_OPEN_CREATENEW, CRYPTCAT_VERSION_2, CryptCATClose, CryptCATOpen, CryptCATPersistStore,
    CryptCATPutAttrInfo, CryptCATPutMemberInfo,
};
use windows::Win32::Security::Cryptography::Sip::{
    CryptSIPCreateIndirectData, SIP_INDIRECT_DATA, SIP_SUBJECTINFO,
};
use windows::Win32::Security::Cryptography::{
    ALG_ID, BCRYPT_RSA_ALGORITHM, CALG_SHA_256, CERT_CONTEXT, CERT_CREATE_SELFSIGN_NO_KEY_INFO,
    CERT_EXTENSION, CERT_EXTENSIONS, CERT_FIND_SUBJECT_STR, CERT_KEY_CONTEXT, CERT_KEY_CONTEXT_0,
    CERT_KEY_CONTEXT_PROP_ID, CERT_KEY_SPEC, CERT_NCRYPT_KEY_SPEC, CERT_OPEN_STORE_FLAGS,
    CERT_QUERY_ENCODING_TYPE, CERT_STORE_ADD_REPLACE_EXISTING, CERT_STORE_NO_CRYPT_RELEASE_FLAG,
    CERT_STORE_PROV_SYSTEM_W, CERT_SYSTEM_STORE_LOCAL_MACHINE_ID, CERT_SYSTEM_STORE_LOCATION_SHIFT,
    CERT_X500_NAME_STR, CRYPT_ALGORITHM_IDENTIFIER, CRYPT_INTEGER_BLOB, CTL_USAGE,
    CertAddCertificateContextToStore, CertCloseStore, CertCreateSelfSignCertificate,
    CertDeleteCertificateFromStore, CertFindCertificateInStore, CertFreeCertificateContext,
    CertOpenStore, CertSetCertificateContextProperty, CertStrToNameW, CryptEncodeObject,
    HCERTSTORE, HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, MS_KEY_STORAGE_PROVIDER,
    NCRYPT_ALLOW_SIGNING_FLAG, NCRYPT_FLAGS, NCRYPT_HANDLE, NCRYPT_KEY_HANDLE,
    NCRYPT_KEY_USAGE_PROPERTY, NCRYPT_LENGTH_PROPERTY, NCRYPT_PROV_HANDLE,
    NCryptCreatePersistedKey, NCryptFinalizeKey, NCryptFreeObject, NCryptOpenStorageProvider,
    NCryptSetProperty, PKCS_7_ASN_ENCODING, SIGNER_CERT, SIGNER_CERT_0, SIGNER_CERT_POLICY_CHAIN,
    SIGNER_CERT_STORE, SIGNER_CERT_STORE_INFO, SIGNER_FILE_INFO, SIGNER_NO_ATTR, SIGNER_SIGN_FLAGS,
    SIGNER_SIGNATURE_INFO, SIGNER_SIGNATURE_INFO_0, SIGNER_SUBJECT_FILE, SIGNER_SUBJECT_INFO,
    SIGNER_SUBJECT_INFO_0, SignerSignEx, X509_ASN_ENCODING, X509_ENHANCED_KEY_USAGE,
    szOID_ENHANCED_KEY_USAGE, szOID_NIST_sha256, szOID_OIWSEC_sha1, szOID_PKIX_KP_CODE_SIGNING,
    szOID_RSA_SHA256RSA,
};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_SHARE_READ, OPEN_EXISTING,
};
use windows::Win32::System::Time::FileTimeToSystemTime;
use windows::core::{GUID, PCSTR, PCWSTR, PSTR};

use super::{Error, Result, wide};

/// Subject of the temporary signing certificate.
///
/// Also the key by which the startup sweep recognises leftovers, so it must
/// stay stable across versions: changing it would orphan certificates written
/// by an older build.
const CERT_SUBJECT: &str = "PMCA Temporary Driver Signing";

/// Machine stores the certificate has to be visible in while staging.
///
/// `Root` makes the chain verifiable at all; `TrustedPublisher` is what turns
/// "unknown publisher, ask the user" into a silent install. Both are cleared
/// again the moment staging is done.
const TRUST_STORES: [&str; 2] = ["Root", "TrustedPublisher"];

const ENCODING: CERT_QUERY_ENCODING_TYPE =
    CERT_QUERY_ENCODING_TYPE(X509_ASN_ENCODING.0 | PKCS_7_ASN_ENCODING.0);

/// `CRYPTCAT_ATTR_NAMEASCII | CRYPTCAT_ATTR_DATAASCII | CRYPTCAT_ATTR_AUTHENTICATED`.
///
/// The name and the data flags live in different nibbles of the same word
/// (`0x1` says the *name* is a wide string, `0x1_0000` says the *value* is);
/// setting two name flags at once is what `ERROR_INVALID_PARAMETER` complains
/// about, and the mistake is easy to make because the constants are not in the
/// `windows` crate and have to be written out by hand.
const CATALOG_ATTR_FLAGS: u32 = 0x0000_0001 | 0x0001_0000 | 0x1000_0000;

/// `CRYPTCAT_MEMBER_VERSION_2`, the certificate version driver catalogs use.
const CATALOG_MEMBER_VERSION: u32 = 0x0000_0200;

/// `CRYPT_SUBJTYPE_FLAT_IMAGE_GUID` — the subject-interface package that hashes
/// a file as a plain byte stream.
///
/// An INF is not one of the structured subject types Windows knows about (PE
/// image, cabinet, catalog, MSI), so `CryptSIPRetrieveSubjectGuid` refuses it
/// with `TRUST_E_SUBJECT_FORM_UNKNOWN`. Every driver-catalog tool names this
/// SIP outright for such members instead of asking, and so do we — for a file
/// whose type we generated ourselves, asking could not tell us anything new.
const FLAT_IMAGE_SIP: GUID = GUID::from_u128(0xde351a42_8e59_11d0_8c47_00c04fc295ee);

/// A certificate that removes itself from both trust stores when it goes out
/// of scope; its in-memory key goes with the context.
///
/// Written as a guard rather than as a pair of calls because the removal has to
/// happen on **every** path out of the staging code, including panics and the
/// early returns of failed staging. A leaked code-signing certificate in
/// `Root` is exactly the kind of residue this whole design exists to avoid.
pub(super) struct TempCertificate {
    context: *mut CERT_CONTEXT,
    /// Dropped after `Drop::drop` has released the context that refers to it.
    _key: EphemeralKey,
}

impl TempCertificate {
    /// Creates a self-signed code-signing certificate and publishes it to the
    /// machine trust stores.
    pub(super) fn create() -> Result<Self> {
        let cert = Self::generate()?;
        for store in TRUST_STORES {
            cert.publish_to(store)?;
        }
        Ok(cert)
    }

    /// Creates the certificate and its key without touching any store.
    fn generate() -> Result<Self> {
        let key = EphemeralKey::generate()?;
        let mut subject = encode_name(&format!("CN={CERT_SUBJECT}"))?;
        let mut eku = encode_code_signing_eku()?;

        let subject_blob =
            CRYPT_INTEGER_BLOB { cbData: subject.len() as u32, pbData: subject.as_mut_ptr() };
        let mut extension = CERT_EXTENSION {
            pszObjId: PSTR(szOID_ENHANCED_KEY_USAGE.0 as *mut u8),
            // Not critical: marking the EKU critical makes some chain engines
            // reject the certificate for the store operations still to come.
            fCritical: false.into(),
            Value: CRYPT_INTEGER_BLOB { cbData: eku.len() as u32, pbData: eku.as_mut_ptr() },
        };
        let extensions = CERT_EXTENSIONS { cExtension: 1, rgExtension: &mut extension };
        let algorithm = CRYPT_ALGORITHM_IDENTIFIER {
            pszObjId: PSTR(szOID_RSA_SHA256RSA.0 as *mut u8),
            Parameters: CRYPT_INTEGER_BLOB::default(),
        };
        let (start, end) = validity_window()?;

        // SAFETY: every pointer above refers to a local that outlives this
        // call, and `key` is a finalised RSA key. NO_KEY_INFO keeps Windows
        // from recording a key container for a key that has none.
        let context = unsafe {
            CertCreateSelfSignCertificate(
                Some(HCRYPTPROV_OR_NCRYPT_KEY_HANDLE(key.handle.0)),
                &subject_blob,
                CERT_CREATE_SELFSIGN_NO_KEY_INFO,
                None,
                Some(&algorithm),
                Some(&start),
                Some(&end),
                Some(&extensions),
            )
        };
        if context.is_null() {
            return Err(Error::last_os("could not create a signing certificate"));
        }
        let handle = key.handle;
        let cert = Self { context, _key: key };

        // Tells the signer which key belongs to the certificate. NO_CRYPT_RELEASE
        // leaves the key with `_key`, which frees it after the context.
        let key_context = CERT_KEY_CONTEXT {
            cbSize: size_of::<CERT_KEY_CONTEXT>() as u32,
            Anonymous: CERT_KEY_CONTEXT_0 { hNCryptKey: handle },
            dwKeySpec: CERT_NCRYPT_KEY_SPEC.0,
        };
        // SAFETY: the context is live and `key_context` outlives the call.
        unsafe {
            CertSetCertificateContextProperty(
                cert.context,
                CERT_KEY_CONTEXT_PROP_ID,
                CERT_STORE_NO_CRYPT_RELEASE_FLAG.0,
                Some(&key_context as *const CERT_KEY_CONTEXT as *const c_void),
            )
        }
        .map_err(|e| Error::Install(format!("could not attach the signing key: {e}")))?;
        Ok(cert)
    }

    pub(super) fn context(&self) -> *const CERT_CONTEXT {
        self.context
    }

    fn publish_to(&self, store_name: &str) -> Result<()> {
        let store = open_machine_store(store_name)?;
        // SAFETY: `self.context` is live and the store handle is closed on
        // both paths below.
        let result = unsafe {
            CertAddCertificateContextToStore(
                Some(store),
                self.context,
                CERT_STORE_ADD_REPLACE_EXISTING,
                None,
            )
        };
        unsafe {
            let _ = CertCloseStore(Some(store), 0);
        }
        result.map_err(|e| {
            store_error(
                &e,
                format!("could not publish the signing certificate to the {store_name} store: {e}"),
            )
        })
    }
}

impl Drop for TempCertificate {
    fn drop(&mut self) {
        // SAFETY: the context is still live here; this is the only place it is
        // released. The key in `_key` is freed right after.
        unsafe {
            let _ = CertFreeCertificateContext(Some(self.context));
        }
        // Swept by subject rather than by handle: if an earlier run died
        // between publishing and cleanup, its certificate shares our subject
        // and should go too.
        let _ = remove_certificates();
    }
}

/// Removes every certificate carrying [`CERT_SUBJECT`] from the machine trust
/// stores and reports how many were found.
///
/// Used both by [`TempCertificate::drop`] and by the startup sweep, which is
/// what makes a crashed run self-healing.
pub(super) fn remove_certificates() -> Result<usize> {
    let mut removed = 0usize;
    for store_name in TRUST_STORES {
        let Ok(store) = open_machine_store(store_name) else {
            continue;
        };
        // SAFETY: the store handle is live and closed immediately after.
        unsafe {
            removed += purge_store(store);
            let _ = CertCloseStore(Some(store), 0);
        }
    }
    Ok(removed)
}

/// # Safety
///
/// `store` must be a live certificate store handle.
unsafe fn purge_store(store: HCERTSTORE) -> usize {
    let needle = wide(CERT_SUBJECT);
    let mut removed = 0usize;
    loop {
        // Deleting a context invalidates the enumeration cursor, so the search
        // restarts from the beginning after every hit instead of carrying a
        // previous-context pointer forward.
        let found = unsafe {
            CertFindCertificateInStore(
                store,
                ENCODING,
                0,
                CERT_FIND_SUBJECT_STR,
                Some(needle.as_ptr() as *const c_void),
                None,
            )
        };
        if found.is_null() {
            return removed;
        }
        // CertDeleteCertificateFromStore frees the context it is given, so it
        // must not be freed again here.
        if unsafe { CertDeleteCertificateFromStore(found) }.is_ok() {
            removed += 1;
        } else {
            unsafe {
                let _ = CertFreeCertificateContext(Some(found));
            }
            return removed;
        }
    }
}

/// An RSA signing key that NCrypt keeps in memory only: a key created without
/// a name is never persisted.
struct EphemeralKey {
    provider: NCRYPT_PROV_HANDLE,
    handle: NCRYPT_KEY_HANDLE,
}

impl EphemeralKey {
    fn generate() -> Result<Self> {
        let fail = |e: windows::core::Error| {
            Error::Install(format!("could not create a signing key: {e}"))
        };
        let mut key =
            Self { provider: NCRYPT_PROV_HANDLE::default(), handle: NCRYPT_KEY_HANDLE::default() };
        // SAFETY: both out-parameters are fields of `key`, whose `Drop`
        // releases whatever was opened if a later step fails.
        unsafe {
            NCryptOpenStorageProvider(&mut key.provider, MS_KEY_STORAGE_PROVIDER, 0)
                .map_err(fail)?;
            NCryptCreatePersistedKey(
                key.provider,
                &mut key.handle,
                BCRYPT_RSA_ALGORITHM,
                PCWSTR::null(),
                CERT_KEY_SPEC(0),
                NCRYPT_FLAGS(0),
            )
            .map_err(fail)?;
            let object = NCRYPT_HANDLE(key.handle.0);
            NCryptSetProperty(
                object,
                NCRYPT_LENGTH_PROPERTY,
                &2048u32.to_le_bytes(),
                NCRYPT_FLAGS(0),
            )
            .map_err(fail)?;
            NCryptSetProperty(
                object,
                NCRYPT_KEY_USAGE_PROPERTY,
                &NCRYPT_ALLOW_SIGNING_FLAG.to_le_bytes(),
                NCRYPT_FLAGS(0),
            )
            .map_err(fail)?;
            NCryptFinalizeKey(key.handle, NCRYPT_FLAGS(0)).map_err(fail)?;
        }
        Ok(key)
    }
}

impl Drop for EphemeralKey {
    fn drop(&mut self) {
        // SAFETY: each handle is either zero or open and owned by this value.
        unsafe {
            if self.handle.0 != 0 {
                let _ = NCryptFreeObject(NCRYPT_HANDLE(self.handle.0));
            }
            if self.provider.0 != 0 {
                let _ = NCryptFreeObject(NCRYPT_HANDLE(self.provider.0));
            }
        }
    }
}

fn open_machine_store(name: &str) -> Result<HCERTSTORE> {
    let flags = CERT_SYSTEM_STORE_LOCAL_MACHINE_ID << CERT_SYSTEM_STORE_LOCATION_SHIFT;
    let wide_name = wide(name);
    // SAFETY: `wide_name` outlives the call; the caller closes the handle.
    unsafe {
        CertOpenStore(
            CERT_STORE_PROV_SYSTEM_W,
            CERT_QUERY_ENCODING_TYPE(0),
            None,
            CERT_OPEN_STORE_FLAGS(flags),
            Some(wide_name.as_ptr() as *const c_void),
        )
    }
    .map_err(|e| {
        store_error(&e, format!("could not open the machine {name} certificate store: {e}"))
    })
}

/// The machine trust stores are writable by administrators only, so this is
/// where a non-elevated run fails first — before staging, which has its own
/// access check. Reported as [`Error::NeedsElevation`] so callers can offer
/// the elevated restart instead of showing a raw Windows message.
fn store_error(e: &windows::core::Error, message: String) -> Error {
    if e.code() == E_ACCESSDENIED { Error::NeedsElevation } else { Error::Install(message) }
}

fn encode_name(subject: &str) -> Result<Vec<u8>> {
    let wide_subject = wide(subject);
    let mut size = 0u32;
    let fail = |e| Error::Install(format!("could not encode the certificate subject: {e}"));

    // SAFETY: two-call idiom — first for the size, then for the data.
    unsafe {
        CertStrToNameW(
            ENCODING,
            PCWSTR(wide_subject.as_ptr()),
            CERT_X500_NAME_STR,
            None,
            None,
            &mut size,
            None,
        )
    }
    .map_err(fail)?;

    let mut buffer = vec![0u8; size as usize];
    unsafe {
        CertStrToNameW(
            ENCODING,
            PCWSTR(wide_subject.as_ptr()),
            CERT_X500_NAME_STR,
            None,
            Some(buffer.as_mut_ptr()),
            &mut size,
            None,
        )
    }
    .map_err(fail)?;
    buffer.truncate(size as usize);
    Ok(buffer)
}

fn encode_code_signing_eku() -> Result<Vec<u8>> {
    let mut oid = PSTR(szOID_PKIX_KP_CODE_SIGNING.0 as *mut u8);
    let usage = CTL_USAGE { cUsageIdentifier: 1, rgpszUsageIdentifier: &mut oid };
    let fail = |e| Error::Install(format!("could not encode the key-usage extension: {e}"));

    let mut size = 0u32;
    // SAFETY: two-call idiom against stack locals that outlive both calls.
    unsafe {
        CryptEncodeObject(
            ENCODING,
            X509_ENHANCED_KEY_USAGE,
            &usage as *const _ as *const c_void,
            None,
            &mut size,
        )
    }
    .map_err(fail)?;

    let mut buffer = vec![0u8; size as usize];
    unsafe {
        CryptEncodeObject(
            ENCODING,
            X509_ENHANCED_KEY_USAGE,
            &usage as *const _ as *const c_void,
            Some(buffer.as_mut_ptr()),
            &mut size,
        )
    }
    .map_err(fail)?;
    buffer.truncate(size as usize);
    Ok(buffer)
}

/// Validity window for the temporary certificate: now to one day ahead.
///
/// The certificate is deleted seconds after it is created, so the window is
/// belt and braces — it caps the damage if the process is killed between
/// creating the certificate and the cleanup guard running.
fn validity_window() -> Result<(SYSTEMTIME, SYSTEMTIME)> {
    /// 100-nanosecond intervals between the FILETIME epoch (1601) and the Unix
    /// epoch (1970).
    const EPOCH_DIFFERENCE: u64 = 116_444_736_000_000_000;
    const ONE_DAY: u64 = 24 * 60 * 60 * 10_000_000;

    let unix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| Error::Install("the system clock is set before 1970".into()))?;
    let start = EPOCH_DIFFERENCE + unix.as_secs() * 10_000_000;
    Ok((to_system_time(start)?, to_system_time(start + ONE_DAY)?))
}

fn to_system_time(ticks: u64) -> Result<SYSTEMTIME> {
    let filetime = FILETIME { dwLowDateTime: ticks as u32, dwHighDateTime: (ticks >> 32) as u32 };
    let mut out = SYSTEMTIME::default();
    // SAFETY: both arguments are stack locals of the expected types.
    unsafe { FileTimeToSystemTime(&filetime, &mut out) }
        .map_err(|e| Error::Install(format!("could not compute the certificate validity: {e}")))?;
    Ok(out)
}

/// Builds a catalog covering `inf` and signs it with `cert`.
///
/// The catalog is what the INF's `CatalogFile=` directive points at; without
/// it the package is refused outright.
pub(super) fn create_signed_catalog(
    inf: &Path,
    catalog: &Path,
    cert: &TempCertificate,
) -> Result<()> {
    // The INF goes into the catalog twice, hashed two different ways, and both
    // are needed.
    //
    // Windows checks a driver package in two passes. The first applies the
    // driver-signing policy and fails for anything that is not WHQL-signed --
    // ours included, and equally a package built by Windows' own
    // New-FileCatalog, so it is not the pass to worry about. The second is
    // plain Authenticode, it is the one that decides, and it looks the member
    // up by its SHA-1 digest. A catalog that only knows the SHA-256 digest is
    // invisible to it: staging stops with "no signature was present in the
    // subject", which reads like a missing signature rather than a missing
    // member and sends you looking in the wrong place entirely.
    //
    // SHA-256 stays because the first pass and every modern tool want it. The
    // SHA-1 entry is a lookup key, not a security claim -- the catalog itself
    // is signed with SHA-256.
    let members = [hash_inf(inf, szOID_NIST_sha256)?, hash_inf(inf, szOID_OIWSEC_sha1)?];
    write_catalog(inf, catalog, &members)?;
    sign_file(catalog, cert)
}

/// The digest blob identifying a catalog member.
struct Member {
    /// Serialised `SIP_INDIRECT_DATA`, handed to `CryptCATPutMemberInfo`.
    blob: Vec<u8>,
    /// Uppercase hex of the file digest, used as the member's reference tag.
    tag: String,
    /// Subject-interface package that produced the digest.
    subject: GUID,
}

fn hash_inf(inf: &Path, algorithm: PCSTR) -> Result<Member> {
    let path = wide(&inf.to_string_lossy());

    // SAFETY: the handle is closed on every path out; the indirect data is
    // fetched with the two-call idiom into a buffer that outlives both calls.
    unsafe {
        let handle = CreateFileW(
            PCWSTR(path.as_ptr()),
            FILE_GENERIC_READ.0,
            FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            None,
        )
        .map_err(|e| Error::Install(format!("could not open the generated INF: {e}")))?;

        let result = hash_open_file(&path, handle, algorithm);
        let _ = CloseHandle(handle);
        result
    }
}

/// # Safety
///
/// `handle` must be an open read handle for the file named by `path`.
unsafe fn hash_open_file(path: &[u16], handle: HANDLE, algorithm: PCSTR) -> Result<Member> {
    let mut subject = FLAT_IMAGE_SIP;

    let mut info = SIP_SUBJECTINFO {
        cbSize: size_of::<SIP_SUBJECTINFO>() as u32,
        pgSubjectType: &mut subject,
        hFile: handle,
        pwsFileName: PCWSTR(path.as_ptr()),
        dwEncodingType: ENCODING.0,
        DigestAlgorithm: CRYPT_ALGORITHM_IDENTIFIER {
            pszObjId: PSTR(algorithm.0 as *mut u8),
            Parameters: CRYPT_INTEGER_BLOB::default(),
        },
        ..Default::default()
    };
    let fail = |e| Error::Install(format!("could not hash the generated INF: {e}"));

    let mut size = 0u32;
    unsafe { CryptSIPCreateIndirectData(&mut info, &mut size, std::ptr::null_mut()) }
        .map_err(fail)?;

    let mut blob = vec![0u8; size as usize];
    unsafe {
        CryptSIPCreateIndirectData(
            &mut info,
            &mut size,
            blob.as_mut_ptr() as *mut SIP_INDIRECT_DATA,
        )
    }
    .map_err(fail)?;

    // SAFETY: the call above filled `blob` with a SIP_INDIRECT_DATA whose
    // Digest points into that same allocation.
    let data = unsafe { &*(blob.as_ptr() as *const SIP_INDIRECT_DATA) };
    let digest =
        unsafe { std::slice::from_raw_parts(data.Digest.pbData, data.Digest.cbData as usize) };
    let tag = digest.iter().map(|b| format!("{b:02X}")).collect();

    Ok(Member { blob, tag, subject })
}

fn write_catalog(inf: &Path, catalog: &Path, members: &[Member]) -> Result<()> {
    let catalog_path = wide(&catalog.to_string_lossy());
    let file_name = inf.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let member_file = wide(&file_name);

    // SAFETY: the catalog handle is closed on every path below, and all
    // buffers outlive the calls that read them.
    unsafe {
        let handle = CryptCATOpen(
            PCWSTR(catalog_path.as_ptr()),
            CRYPTCAT_OPEN_CREATENEW,
            0,
            CRYPTCAT_VERSION_2,
            ENCODING.0,
        );
        if handle.is_invalid() {
            return Err(Error::last_os("could not create the driver catalog"));
        }

        let result = fill_catalog(handle, members, &member_file).and_then(|()| {
            CryptCATPersistStore(handle)
                .map_err(|e| Error::Install(format!("could not write the driver catalog: {e}")))
        });
        let _ = CryptCATClose(handle);
        result
    }
}

/// # Safety
///
/// `handle` must be an open catalog handle.
unsafe fn fill_catalog(handle: HANDLE, members: &[Member], file_name: &[u16]) -> Result<()> {
    for member in members {
        // SAFETY: forwarding the caller's guarantee about `handle`.
        unsafe { put_member(handle, member, file_name)? };
    }
    Ok(())
}

/// Adds `member` to the catalog under its own reference tag.
///
/// # Safety
///
/// `handle` must be an open catalog handle.
unsafe fn put_member(handle: HANDLE, member: &Member, file_name: &[u16]) -> Result<()> {
    let mut subject = member.subject;
    let tag = wide(&member.tag);
    let entry = unsafe {
        CryptCATPutMemberInfo(
            handle,
            PCWSTR(file_name.as_ptr()),
            PCWSTR(tag.as_ptr()),
            &mut subject,
            CATALOG_MEMBER_VERSION,
            member.blob.len() as u32,
            member.blob.as_ptr() as *mut u8,
        )
    };
    if entry.is_null() {
        return Err(Error::last_os("could not add the INF to the driver catalog"));
    }

    // The `FilePath` attribute is what lets the verifier report *which* file
    // failed when a package is tampered with. The name is not free-form:
    // Windows' own tools use exactly this one, and diagnostics elsewhere look
    // for it.
    let attr_name = wide("FilePath");
    let attribute = unsafe {
        CryptCATPutAttrInfo(
            handle,
            entry,
            PCWSTR(attr_name.as_ptr()),
            CATALOG_ATTR_FLAGS,
            (file_name.len() * 2) as u32,
            file_name.as_ptr() as *mut u8,
        )
    };
    if attribute.is_null() {
        return Err(Error::last_os("could not annotate the driver catalog"));
    }

    Ok(())
}

fn sign_file(file: &Path, cert: &TempCertificate) -> Result<()> {
    let path = wide(&file.to_string_lossy());
    let mut index = 0u32;

    let mut file_info = SIGNER_FILE_INFO {
        cbSize: size_of::<SIGNER_FILE_INFO>() as u32,
        pwszFileName: PCWSTR(path.as_ptr()),
        hFile: HANDLE::default(),
    };
    let subject = SIGNER_SUBJECT_INFO {
        cbSize: size_of::<SIGNER_SUBJECT_INFO>() as u32,
        pdwIndex: &mut index,
        dwSubjectChoice: SIGNER_SUBJECT_FILE,
        Anonymous: SIGNER_SUBJECT_INFO_0 { pSignerFileInfo: &mut file_info },
    };

    let mut store_info = SIGNER_CERT_STORE_INFO {
        cbSize: size_of::<SIGNER_CERT_STORE_INFO>() as u32,
        pSigningCert: cert.context(),
        dwCertPolicy: SIGNER_CERT_POLICY_CHAIN,
        hCertStore: HCERTSTORE::default(),
    };
    let signer_cert = SIGNER_CERT {
        cbSize: size_of::<SIGNER_CERT>() as u32,
        dwCertChoice: SIGNER_CERT_STORE,
        Anonymous: SIGNER_CERT_0 { pCertStoreInfo: &mut store_info },
        hwnd: Default::default(),
    };

    let signature = SIGNER_SIGNATURE_INFO {
        cbSize: size_of::<SIGNER_SIGNATURE_INFO>() as u32,
        algidHash: ALG_ID(CALG_SHA_256.0),
        dwAttrChoice: SIGNER_NO_ATTR,
        Anonymous: SIGNER_SIGNATURE_INFO_0 { pAttrAuthcode: std::ptr::null_mut() },
        psAuthenticated: std::ptr::null_mut(),
        psUnauthenticated: std::ptr::null_mut(),
    };

    // SAFETY: every structure above is a stack local that outlives the call.
    // No timestamp server is contacted: the certificate is gone within
    // seconds, so a countersignature would be meaningless.
    unsafe {
        SignerSignEx(
            SIGNER_SIGN_FLAGS(0),
            &subject,
            &signer_cert,
            &signature,
            None,
            PCWSTR::null(),
            None,
            None,
        )
    }
    .map(|_| ())
    .map_err(|e| Error::Install(format!("could not sign the driver catalog: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The in-memory key signs a real catalog. Needs no elevation: no store
    /// is touched.
    #[test]
    fn an_in_memory_key_signs_the_catalog() {
        let dir = std::env::temp_dir().join(format!("pmca-sign-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp directory");
        let inf = dir.join("pmca_winusb.inf");
        std::fs::write(&inf, b"; not a real INF, only something to hash\r\n").expect("write INF");
        let catalog = dir.join("pmca_winusb.cat");
        let members = [hash_inf(&inf, szOID_NIST_sha256).expect("SHA-256 digest")];
        write_catalog(&inf, &catalog, &members).expect("write the catalog");
        let unsigned = std::fs::metadata(&catalog).expect("catalog").len();

        let cert = TempCertificate::generate().expect("certificate with an in-memory key");
        let signed = sign_file(&catalog, &cert);
        let signed_len = std::fs::metadata(&catalog).map(|m| m.len()).unwrap_or(0);
        drop(cert);
        let _ = std::fs::remove_dir_all(&dir);

        signed.expect("sign the catalog");
        assert!(signed_len > unsigned, "the signature did not grow the catalog");
    }

    /// Builds a catalog for real and checks that both members survive.
    ///
    /// Catalog creation needs no elevation and touches nothing outside the
    /// temp directory, so this can run in the normal test suite -- which
    /// matters, because the two-digest requirement is invisible in the API and
    /// the only other way to notice a regression is a driver installation
    /// failing on a camera with a misleading message.
    #[test]
    fn catalog_holds_both_digests() {
        let dir = std::env::temp_dir().join(format!("pmca-catalog-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp directory");
        let inf = dir.join("pmca_winusb.inf");
        std::fs::write(&inf, b"; not a real INF, only something to hash\r\n").expect("write INF");
        let catalog = dir.join("pmca_winusb.cat");

        let members = [
            hash_inf(&inf, szOID_NIST_sha256).expect("SHA-256 digest"),
            hash_inf(&inf, szOID_OIWSEC_sha1).expect("SHA-1 digest"),
        ];
        write_catalog(&inf, &catalog, &members).expect("write the catalog");

        let bytes = std::fs::read(&catalog).expect("read the catalog");
        let _ = std::fs::remove_dir_all(&dir);

        for member in &members {
            let digest: Vec<u8> = (0..member.tag.len() / 2)
                .map(|i| u8::from_str_radix(&member.tag[i * 2..i * 2 + 2], 16).expect("hex digest"))
                .collect();
            assert!(
                bytes.windows(digest.len()).any(|w| w == digest.as_slice()),
                "catalog is missing the {}-byte digest",
                digest.len()
            );
        }
    }
}
