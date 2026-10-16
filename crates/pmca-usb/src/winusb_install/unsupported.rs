//! Stand-in for [`crate::winusb_install`] where driver installation does not
//! apply: every platform other than Windows, and Windows builds without the
//! `hardware` feature.
//!
//! It exists so callers do not have to carry `#[cfg(windows)]` around the
//! cleanup calls. Linux and macOS have no equivalent problem to solve — libusb
//! reaches the camera without replacing a kernel driver — so "nothing to clean
//! up" is the correct answer here, not a missing feature.

use std::fmt;

/// Errors from driver installation. Never produced on this platform.
#[derive(Debug)]
pub enum Error {
    /// The operation needs administrator rights.
    NeedsElevation,
    /// The hardware id is not a plain USB vendor/product id.
    BadHardwareId(String),
    /// A driver-installation call failed.
    Install(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NeedsElevation => {
                f.write_str("changing driver assignments requires administrator rights")
            }
            Self::BadHardwareId(id) => write!(f, "not a usable USB hardware id: {id}"),
            Self::Install(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for Error {}

/// Result alias for this module.
pub type Result<T> = std::result::Result<T, Error>;

/// What a cleanup pass removed. Always empty on this platform.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Cleanup {
    /// Published names of the packages removed.
    pub packages: Vec<String>,
    /// Number of leftover signing certificates removed.
    pub certificates: usize,
}

impl Cleanup {
    /// Whether anything was actually found.
    pub fn is_empty(&self) -> bool {
        self.packages.is_empty() && self.certificates == 0
    }
}

/// Binds WinUSB to a hardware id. Not applicable on this platform.
pub fn bind(_hardware_id: &str) -> Result<String> {
    Err(Error::Install("driver assignment is only needed on Windows".into()))
}

/// Binds WinUSB if the device needs it. Never does here: libusb reaches the
/// camera without displacing a kernel driver, so nothing ever needs binding.
pub fn needs_binding(_current_driver: Option<&str>) -> bool {
    false
}

/// Whether a driverless device is plugged in. Always `false` here: nothing
/// asks, because nothing on this platform has to bind a driver before the USB
/// backend can see a device.
pub fn device_present(_hardware_id: &str) -> bool {
    false
}

/// Removes every driver package this crate staged. Nothing to do here.
pub fn unbind_all() -> Result<Cleanup> {
    Ok(Cleanup::default())
}

/// Startup sweep. Nothing to do here.
pub fn sweep() -> Cleanup {
    Cleanup::default()
}

/// Driver packages still staged. Never any here.
pub fn pending_packages() -> usize {
    0
}

/// Whether drivers can be assigned. Always, since none ever has to be.
pub fn can_assign_drivers() -> bool {
    true
}

/// Restarts elevated. Not applicable on this platform, where the raw-USB paths
/// need access to a device node rather than administrator rights.
pub fn launch_elevated(_arguments: &str, _parent: Option<isize>) -> Result<u32> {
    Err(Error::Install("an elevated helper is only meaningful on Windows".into()))
}

/// The pipe to the elevated helper. Never used on this platform.
pub mod helper_pipe {
    use super::{Error, Result};

    /// Full path of a pipe called `name`.
    pub fn pipe_path(name: &str) -> String {
        name.to_string()
    }

    /// Not available on this platform.
    pub fn current_user_sid() -> Result<String> {
        Err(Error::Install("only on Windows".into()))
    }

    /// Not available on this platform.
    pub fn serve(_name: &str, _user_sid: &str, _client_pid: u32) -> Result<std::fs::File> {
        Err(Error::Install("only on Windows".into()))
    }

    /// Not available on this platform.
    pub fn server_pid(_pipe: &std::fs::File) -> Result<u32> {
        Err(Error::Install("only on Windows".into()))
    }

    /// Nothing to wait for on this platform.
    pub fn wait_for_exit(_pid: u32) {}
}
