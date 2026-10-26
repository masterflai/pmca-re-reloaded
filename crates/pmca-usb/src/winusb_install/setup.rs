//! The SetupAPI half: staging a driver package, binding it, removing it again.
//!
//! Split from the signing code because the two have different trust
//! lifetimes — the certificate matters only while [`stage_package`] runs,
//! whereas the staged package and its binding outlive it.

use std::path::Path;

use windows::Win32::Devices::DeviceAndDriverInstallation::{
    CM_GETIDLIST_FILTER_ENUMERATOR, CM_GETIDLIST_FILTER_PRESENT, CM_Get_Device_ID_List_SizeW,
    CM_Get_Device_ID_ListW, CM_LOCATE_DEVNODE_NORMAL, CM_Locate_DevNodeW, CM_REENUMERATE_NORMAL,
    CM_Reenumerate_DevNode, CONFIGRET, CR_SUCCESS, DIGCF_ALLCLASSES, DIIRFLAG_FORCE_INF,
    DIUNINSTALLDRIVER_FLAGS, DiInstallDriverW, DiUninstallDevice, DiUninstallDriverW, HDEVINFO,
    OEM_SOURCE_MEDIA_TYPE, SP_DEVINFO_DATA, SPDRP_MFG, SPOST_PATH, SetupCopyOEMInfW,
    SetupDiDestroyDeviceInfoList, SetupDiEnumDeviceInfo, SetupDiGetClassDevsW,
    SetupDiGetDeviceInstanceIdW, SetupDiGetDeviceRegistryPropertyW, SetupUninstallOEMInfW,
};
use windows::Win32::Foundation::{ERROR_ACCESS_DENIED, HWND};
use windows::core::PCWSTR;

use super::{Error, PROVIDER, Result, wide};

/// `SP_COPY_NEWER_ONLY` — do not re-copy an identical package.
const SP_COPY_NEWER_ONLY: u32 = 0x0000_0004;

/// `SUOI_FORCEDELETE` — remove the package even while devices still reference
/// it. Without this the removal silently does nothing whenever the camera is
/// still plugged in, which is the normal case at service-shell exit.
const SUOI_FORCEDELETE: u32 = 0x0000_0001;

/// Copies the package into the DriverStore and returns its published name.
///
/// This is the only step that checks the signature, and therefore the only
/// step the temporary certificate has to be alive for.
pub(super) fn stage_package(inf: &Path) -> Result<String> {
    let path = wide(&inf.to_string_lossy());
    let mut destination = [0u16; 260];
    let mut component = windows::core::PWSTR::null();

    // SAFETY: the buffer and path outlive the call; `component` is filled with
    // a pointer into `destination`.
    let result = unsafe {
        SetupCopyOEMInfW(
            PCWSTR(path.as_ptr()),
            PCWSTR::null(),
            OEM_SOURCE_MEDIA_TYPE(SPOST_PATH.0),
            windows::Win32::Devices::DeviceAndDriverInstallation::SP_COPY_STYLE(SP_COPY_NEWER_ONLY),
            Some(&mut destination),
            None,
            Some(&mut component),
        )
    };

    if let Err(error) = result {
        if error.code().0 as u32 == 0x8007_0000 | ERROR_ACCESS_DENIED.0 {
            return Err(Error::NeedsElevation);
        }
        return Err(Error::Install(format!(
            "Windows refused to store the driver package: {error}"
        )));
    }

    // SAFETY: on success `component` points into `destination`, which is still
    // in scope.
    let published = unsafe { component.to_string() }.unwrap_or_default();
    if published.is_empty() {
        return Err(Error::Install(
            "Windows stored the driver package but did not report its name".into(),
        ));
    }
    Ok(published)
}

/// Binds the already staged package to whatever devices its INF matches.
///
/// Returns whether Windows asked for a reboot — measured as `false` for this
/// package, but reported rather than assumed.
pub(super) fn install_package(inf: &Path) -> Result<bool> {
    let path = wide(&inf.to_string_lossy());
    let mut reboot = windows::core::BOOL::default();

    // SAFETY: `path` outlives the call. `DIIRFLAG_FORCE_INF` selects this
    // package even when Windows prefers its own ranking — which it does here,
    // since the in-box mass-storage driver outranks a generic WinUSB package.
    unsafe { DiInstallDriverW(None, PCWSTR(path.as_ptr()), DIIRFLAG_FORCE_INF, Some(&mut reboot)) }
        .map_err(|e| Error::Install(format!("Windows refused to assign the driver: {e}")))?;

    Ok(reboot.as_bool())
}

/// Removes a staged package, releasing any device bound to it.
///
/// Deleting the package alone is not enough: a device bound to it keeps the
/// binding and stays on WinUSB with a driver that is no longer in the store.
/// `DiUninstallDriverW` is the step that hands the device back to the driver
/// Windows ships, and on success it removes the package as well — which is why
/// the second call is a fallback, not a sequel. It is reached only for a
/// package no device ever bound to, where the first call has nothing to undo.
pub(super) fn remove_package(published_name: &str) -> Result<()> {
    // The two calls disagree about what names a package: `SetupUninstallOEMInfW`
    // wants the bare published name, `DiUninstallDriverW` a path it can open.
    let full_path = super::inf_directory()
        .map(|dir| dir.join(published_name))
        .map(|p| wide(&p.to_string_lossy()));

    // SAFETY: both buffers outlive their calls.
    unsafe {
        if let Some(path) = &full_path
            && DiUninstallDriverW(None, PCWSTR(path.as_ptr()), DIUNINSTALLDRIVER_FLAGS(0), None)
                .is_ok()
        {
            return Ok(());
        }

        let name = wide(published_name);
        if !SetupUninstallOEMInfW(PCWSTR(name.as_ptr()), SUOI_FORCEDELETE, None).as_bool() {
            return Err(Error::last_os(&format!(
                "could not remove the driver package {published_name}"
            )));
        }
    }
    Ok(())
}

/// Asks Windows to re-evaluate driver assignments.
///
/// After removing a package the affected device keeps its now-dangling binding
/// until something triggers a re-enumeration; without this the camera stays
/// driverless until it is unplugged.
pub(super) fn rescan_devices() {
    let mut root = 0u32;
    // SAFETY: both calls take plain out-parameters. Failure is not actionable
    // — the device recovers on the next re-plug — so the result is dropped.
    unsafe {
        if CM_Locate_DevNodeW(&mut root, PCWSTR::null(), CM_LOCATE_DEVNODE_NORMAL) == CR_SUCCESS {
            let _: CONFIGRET = CM_Reenumerate_DevNode(root, CM_REENUMERATE_NORMAL);
        }
    }
}

/// Sony device nodes, plugged in or not, still assigned by a package of ours.
///
/// Removing a package rolls back only the devices plugged in at that moment.
/// Right after leaving service mode the camera is between two product ids, so
/// the node of the mode it returns in keeps WinUSB with its package gone; as a
/// mass-storage device it then shows "check the connected device". Measured on
/// a DSC-RX1R. Service-mode nodes are skipped: WinUSB is what they need anyway.
fn orphaned(mut visit: impl FnMut(HDEVINFO, &SP_DEVINFO_DATA)) -> usize {
    let enumerator = wide("USB");
    // SAFETY: `enumerator` outlives the call. Without `DIGCF_PRESENT` the set
    // includes nodes of devices that are not plugged in.
    let Ok(set) = (unsafe {
        SetupDiGetClassDevsW(None, PCWSTR(enumerator.as_ptr()), None, DIGCF_ALLCLASSES)
    }) else {
        return 0;
    };

    let mut found = 0;
    let mut data =
        SP_DEVINFO_DATA { cbSize: size_of::<SP_DEVINFO_DATA>() as u32, ..Default::default() };
    for index in 0.. {
        // SAFETY: `set` is valid until destroyed below; `data` carries its size.
        if unsafe { SetupDiEnumDeviceInfo(set, index, &mut data) }.is_err() {
            break;
        }
        let mut id = [0u16; 256];
        // SAFETY: `id` is a writable buffer of the length passed with it.
        if unsafe { SetupDiGetDeviceInstanceIdW(set, &data, Some(&mut id), None) }.is_err() {
            continue;
        }
        let mut mfg = [0u8; 256];
        // SAFETY: as above, for the manufacturer string.
        if unsafe {
            SetupDiGetDeviceRegistryPropertyW(set, &data, SPDRP_MFG, None, Some(&mut mfg), None)
        }
        .is_err()
        {
            continue;
        }
        if is_orphan(&utf16_until_nul(&id), &utf16_bytes_until_nul(&mfg)) {
            found += 1;
            visit(set, &data);
        }
    }
    // SAFETY: `set` came from `SetupDiGetClassDevsW` and is not used afterwards.
    unsafe {
        let _ = SetupDiDestroyDeviceInfoList(set);
    }
    found
}

fn is_orphan(instance_id: &str, manufacturer: &str) -> bool {
    let id = instance_id.to_ascii_uppercase();
    let Some(rest) = id.strip_prefix(r"USB\VID_054C&PID_") else { return false };
    let senser = u16::from_str_radix(rest.get(..4).unwrap_or_default(), 16)
        .is_ok_and(|pid| crate::sony::SONY_ID_PRODUCT_SENSER.contains(&pid));
    !senser && manufacturer == PROVIDER
}

fn utf16_until_nul(buffer: &[u16]) -> String {
    let end = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
    String::from_utf16_lossy(&buffer[..end])
}

fn utf16_bytes_until_nul(bytes: &[u8]) -> String {
    let units: Vec<u16> = bytes.as_chunks::<2>().0.iter().map(|p| u16::from_le_bytes(*p)).collect();
    utf16_until_nul(&units)
}

/// How many nodes [`remove_orphans`] would remove. Needs no rights.
pub(super) fn count_orphans() -> usize {
    orphaned(|_, _| ())
}

/// Uninstall every node [`count_orphans`] reports, so the next arrival of the
/// device gets the driver Windows ranks best. Needs administrator rights.
pub(super) fn remove_orphans() -> usize {
    let mut removed = 0;
    orphaned(|set, data| {
        // SAFETY: `set` and `data` are valid for the duration of the visit.
        if unsafe { DiUninstallDevice(HWND::default(), set, data, 0, None) }.is_ok() {
            removed += 1;
        }
    });
    removed
}

/// Whether a device matching `hardware_id` is plugged in right now.
///
/// Answers the question the USB backend cannot: its enumeration only reports
/// devices that already have a driver exposing the USB device interface, so a
/// device Windows has no driver for at all is invisible to it. That is exactly
/// the state the camera is in the instant it enters service mode — present,
/// and unreachable until something binds a driver to it.
///
/// The configuration manager has no such blind spot: it lists device nodes,
/// driver or not.
pub(super) fn device_present(hardware_id: &str) -> bool {
    let filter = wide(hardware_id);
    let flags = CM_GETIDLIST_FILTER_ENUMERATOR | CM_GETIDLIST_FILTER_PRESENT;

    let mut length = 0u32;
    // SAFETY: `filter` outlives both calls; the first sizes the buffer the
    // second fills.
    unsafe {
        if CM_Get_Device_ID_List_SizeW(&mut length, PCWSTR(filter.as_ptr()), flags) != CR_SUCCESS {
            return false;
        }
        // An empty list is two NULs: the terminator of the (absent) first
        // entry plus the terminator of the list.
        if length <= 2 {
            return false;
        }

        let mut buffer = vec![0u16; length as usize];
        if CM_Get_Device_ID_ListW(PCWSTR(filter.as_ptr()), &mut buffer, flags) != CR_SUCCESS {
            return false;
        }
        buffer.first().is_some_and(|&c| c != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_non_service_sony_nodes_of_ours_are_orphans() {
        assert!(is_orphan(r"USB\VID_054C&PID_0586\C6C7805BD4C3", PROVIDER));
        assert!(is_orphan(r"usb\vid_054c&pid_03e2\X", PROVIDER));
        assert!(!is_orphan(r"USB\VID_054C&PID_0336\X", PROVIDER));
        assert!(!is_orphan(r"USB\VID_054C&PID_0586\X", "Sony"));
        assert!(!is_orphan(r"USB\VID_1234&PID_0586\X", PROVIDER));
    }

    #[test]
    fn registry_strings_end_at_the_first_nul() {
        let bytes: Vec<u8> = "AB\0C".encode_utf16().flat_map(u16::to_le_bytes).collect();
        assert_eq!(utf16_bytes_until_nul(&bytes), "AB");
    }
}
