//! Windows services known to get in the way of a camera.
//!
//! The 0xPIT fork of the reference added a diagnostic for one of them, the
//! Windows Media Player Network Sharing Service, which can hold on to a camera
//! in MTP mode (upstream issue #41). Its check ran `tasklist` for an image name
//! the service does not use (`wmpnetwk.exe` is the process, `WMPNetworkSvc`
//! the service), so it could never fire. Here the service control manager is
//! asked directly, which needs no extra process and no console window.
//!
//! Only reading is done: the user is told what is running and how to stop it,
//! but stopping a system service is left to them.

/// A running service that is known to interfere with camera access.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunningService {
    /// The service's key name, as `net stop` and `services.msc` know it.
    pub name: String,
    /// The name Windows shows for it, in the system's language.
    pub display_name: String,
}

/// Service key names to look for.
#[cfg_attr(not(all(windows, feature = "hardware")), allow(dead_code))]
const KNOWN: &[&str] = &["WMPNetworkSvc"];

/// The known interfering services that are currently running.
///
/// A service that is not installed, or whose state cannot be read, counts as
/// not running: this is advice, and a check that cannot be made is not a
/// reason to warn about something.
#[must_use]
pub fn interfering_services() -> Vec<RunningService> {
    #[cfg(all(windows, feature = "hardware"))]
    {
        imp::running(KNOWN)
    }
    #[cfg(not(all(windows, feature = "hardware")))]
    {
        Vec::new()
    }
}

#[cfg(all(windows, feature = "hardware"))]
#[allow(unsafe_code)]
mod imp {
    use windows::Win32::System::Services::{
        CloseServiceHandle, GetServiceDisplayNameW, OpenSCManagerW, OpenServiceW,
        QueryServiceStatus, SC_HANDLE, SC_MANAGER_CONNECT, SERVICE_QUERY_STATUS, SERVICE_STATUS,
        SERVICE_STOPPED,
    };
    use windows::core::{PCWSTR, PWSTR};

    use super::RunningService;

    /// Closes a service-manager handle when it goes out of scope.
    struct Handle(SC_HANDLE);

    impl Drop for Handle {
        fn drop(&mut self) {
            // SAFETY: the handle came from a successful Open* call and is
            // closed exactly once, here.
            let _ = unsafe { CloseServiceHandle(self.0) };
        }
    }

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    pub(super) fn running(names: &[&str]) -> Vec<RunningService> {
        // SAFETY: null machine and database names select the local default
        // database; SC_MANAGER_CONNECT is the least access that allows
        // OpenServiceW and needs no administrator rights.
        let Ok(manager) =
            (unsafe { OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_CONNECT) })
        else {
            return Vec::new();
        };
        let manager = Handle(manager);
        names
            .iter()
            .filter(|name| is_running(&manager, name))
            .map(|name| describe(&manager, name))
            .collect()
    }

    fn is_running(manager: &Handle, name: &str) -> bool {
        let key = wide(name);
        // SAFETY: `key` is NUL-terminated and outlives the call.
        let Ok(service) =
            (unsafe { OpenServiceW(manager.0, PCWSTR(key.as_ptr()), SERVICE_QUERY_STATUS) })
        else {
            return false;
        };
        let service = Handle(service);
        let mut status = SERVICE_STATUS::default();
        // SAFETY: `status` is a valid, writable SERVICE_STATUS.
        if unsafe { QueryServiceStatus(service.0, &mut status) }.is_err() {
            return false;
        }
        status.dwCurrentState != SERVICE_STOPPED
    }

    fn describe(manager: &Handle, name: &str) -> RunningService {
        let key = wide(name);
        let mut buffer = [0u16; 256];
        let mut len = buffer.len() as u32;
        // SAFETY: `key` is NUL-terminated; `buffer` holds `len` characters,
        // and on success `len` is set to the length without the terminator.
        let display_name = match unsafe {
            GetServiceDisplayNameW(
                manager.0,
                PCWSTR(key.as_ptr()),
                Some(PWSTR(buffer.as_mut_ptr())),
                &mut len,
            )
        } {
            Ok(()) => String::from_utf16_lossy(&buffer[..(len as usize).min(buffer.len())]),
            Err(_) => name.to_string(),
        };
        RunningService { name: name.to_string(), display_name }
    }
}
