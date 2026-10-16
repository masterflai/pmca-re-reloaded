//! Sony vendor/product constants and device-detection helpers.
//!
//! Port of the identification logic in `pmca/usb/sony.py`. Detection is pure
//! data inspection, so it is fully unit-testable without hardware.

/// Sony USB vendor id.
pub const SONY_ID_VENDOR: u16 = 0x054c;

/// Product ids for cameras in firmware-updater mode.
pub const SONY_ID_PRODUCT_UPDATER: &[u16] = &[0x033a, 0x03e2];

/// Product ids for cameras in service/senser mode.
pub const SONY_ID_PRODUCT_SENSER: &[u16] = &[0x02a9, 0x0336];

/// Manufacturer string reported in MTP mode.
pub const SONY_MANUFACTURER: &str = "Sony Corporation";

/// Manufacturer string reported in mass-storage mode.
pub const SONY_MANUFACTURER_SHORT: &str = "Sony";

/// Known model strings reported by cameras in mass-storage mode.
pub const SONY_MSC_MODELS: &[&str] = &["DSC", "Camcorder"];

/// Device info reported by a mass-storage (SCSI INQUIRY) device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MscDeviceInfo {
    /// Vendor / manufacturer string.
    pub manufacturer: String,
    /// Product / model string.
    pub model: String,
}

/// Device info reported by a PTP/MTP device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MtpDeviceInfo {
    /// Manufacturer string.
    pub manufacturer: String,
    /// Model string.
    pub model: String,
    /// Serial number string.
    pub serial_number: String,
    /// Set of supported PTP operation codes.
    pub operations_supported: Vec<u16>,
    /// Vendor-extension descriptor string.
    pub vendor_extension: String,
}

/// PTP operation codes used to recognize Sony camera MTP modes.
pub mod ptp_op {
    /// Sony ext-cmd write.
    pub const SONY_DI_EXT_CMD_WRITE: u16 = 0x9280;
    /// Sony ext-cmd read.
    pub const SONY_DI_EXT_CMD_READ: u16 = 0x9281;
    /// Sony reconnect request.
    pub const SONY_REQ_RECONNECT: u16 = 0x9282;
    /// App-install: get proxy message info.
    pub const GET_PROXY_MESSAGE_INFO: u16 = 0x9488;
    /// App-install: get proxy message.
    pub const GET_PROXY_MESSAGE: u16 = 0x9489;
    /// App-install: send proxy message info.
    pub const SEND_PROXY_MESSAGE_INFO: u16 = 0x948c;
    /// App-install: send proxy message.
    pub const SEND_PROXY_MESSAGE: u16 = 0x948d;
}

/// Returns `true` if a mass-storage device looks like a Sony camera.
pub fn is_sony_msc_camera(info: &MscDeviceInfo) -> bool {
    info.manufacturer == SONY_MANUFACTURER_SHORT && SONY_MSC_MODELS.iter().any(|m| *m == info.model)
}

/// Returns `true` for a camera in firmware-updater mass-storage mode.
pub fn is_sony_msc_updater_camera(vendor: u16, product: u16) -> bool {
    vendor == SONY_ID_VENDOR && SONY_ID_PRODUCT_UPDATER.contains(&product)
}

/// Returns `true` for a camera in service/senser mode.
pub fn is_sony_senser_camera(vendor: u16, product: u16) -> bool {
    vendor == SONY_ID_VENDOR && SONY_ID_PRODUCT_SENSER.contains(&product)
}

fn supports_all(info: &MtpDeviceInfo, ops: &[u16]) -> bool {
    ops.iter().all(|op| info.operations_supported.contains(op))
}

/// Returns `true` for a Sony camera in ext-cmd MTP mode.
pub fn is_sony_mtp_camera(info: &MtpDeviceInfo) -> bool {
    info.manufacturer == SONY_MANUFACTURER
        && info.vendor_extension.is_empty()
        && supports_all(
            info,
            &[
                ptp_op::SONY_DI_EXT_CMD_WRITE,
                ptp_op::SONY_DI_EXT_CMD_READ,
                ptp_op::SONY_REQ_RECONNECT,
            ],
        )
}

/// Returns `true` for a Sony camera in app-installation MTP mode.
pub fn is_sony_mtp_app_install_camera(info: &MtpDeviceInfo) -> bool {
    info.manufacturer == SONY_MANUFACTURER
        && info.vendor_extension.contains("sony.net/SEN_PRXY_MSG:")
        && supports_all(
            info,
            &[
                ptp_op::GET_PROXY_MESSAGE_INFO,
                ptp_op::GET_PROXY_MESSAGE,
                ptp_op::SEND_PROXY_MESSAGE_INFO,
                ptp_op::SEND_PROXY_MESSAGE,
            ],
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn msc_camera_detection() {
        let cam = MscDeviceInfo { manufacturer: "Sony".into(), model: "DSC".into() };
        assert!(is_sony_msc_camera(&cam));
        let other = MscDeviceInfo { manufacturer: "Sony".into(), model: "Phone".into() };
        assert!(!is_sony_msc_camera(&other));
    }

    #[test]
    fn updater_and_senser_by_pid() {
        assert!(is_sony_msc_updater_camera(SONY_ID_VENDOR, 0x033a));
        assert!(!is_sony_msc_updater_camera(SONY_ID_VENDOR, 0x0000));
        assert!(is_sony_senser_camera(SONY_ID_VENDOR, 0x02a9));
        assert!(!is_sony_senser_camera(0x1234, 0x02a9));
    }

    #[test]
    fn mtp_extcmd_camera_needs_all_ops_and_empty_extension() {
        let ok = MtpDeviceInfo {
            manufacturer: SONY_MANUFACTURER.into(),
            model: "ILCE".into(),
            serial_number: "123".into(),
            operations_supported: vec![0x9280, 0x9281, 0x9282, 0x1001],
            vendor_extension: String::new(),
        };
        assert!(is_sony_mtp_camera(&ok));

        let missing_op = MtpDeviceInfo { operations_supported: vec![0x9280, 0x9281], ..ok.clone() };
        assert!(!is_sony_mtp_camera(&missing_op));
    }

    #[test]
    fn mtp_app_install_needs_extension_marker() {
        let info = MtpDeviceInfo {
            manufacturer: SONY_MANUFACTURER.into(),
            model: "ILCE".into(),
            serial_number: "1".into(),
            operations_supported: vec![0x9488, 0x9489, 0x948c, 0x948d],
            vendor_extension: "sony.net/SEN_PRXY_MSG: 1.0".into(),
        };
        assert!(is_sony_mtp_app_install_camera(&info));
        assert!(!is_sony_mtp_camera(&info)); // extension not empty
    }
}
