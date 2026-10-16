//! Console setup: UTF-8 code page on Windows and the user's language.

/// Makes the Windows console interpret our output as UTF-8, and puts it back.
///
/// Rust writes UTF-8, but a Windows console decodes byte output using its
/// configured output code page, which is a legacy 8-bit one by default (850 on
/// a German install). Without this every non-ASCII character in a translated
/// message is rendered as mojibake — Japanese becomes unreadable entirely, and
/// German umlauts are silently corrupted.
///
/// The code page is a property of the console, not of this process, so a change
/// outlives us and would corrupt the output of whatever the user runs next.
/// [`ConsoleCodePage`] therefore restores the previous setting on drop.
///
/// Failures are deliberately ignored: when stdout is a pipe or a file there is
/// no console to configure, and that is not an error.
#[cfg(windows)]
pub(crate) struct ConsoleCodePage {
    /// The output code page that was configured before we changed it.
    previous_output: u32,
    /// The input code page that was configured before we changed it.
    previous_input: u32,
}

#[cfg(windows)]
pub(crate) mod console_ffi {
    //! The binary's only FFI, mirroring `pmca_usb::scsi_passthrough` in style.
    #[link(name = "kernel32")]
    unsafe extern "system" {
        pub fn GetConsoleOutputCP() -> u32;
        pub fn GetConsoleCP() -> u32;
        pub fn SetConsoleOutputCP(code_page: u32) -> i32;
        pub fn SetConsoleCP(code_page: u32) -> i32;
        pub fn GetUserDefaultLocaleName(locale_name: *mut u16, length: i32) -> i32;
    }
}

#[cfg(windows)]
impl ConsoleCodePage {
    /// Windows code page identifier for UTF-8.
    const UTF8: u32 = 65001;

    /// Switches the console to UTF-8, remembering what it was set to.
    pub(crate) fn switch_to_utf8() -> Self {
        // SAFETY: plain Win32 calls taking and returning integers only.
        unsafe {
            let guard = Self {
                previous_output: console_ffi::GetConsoleOutputCP(),
                previous_input: console_ffi::GetConsoleCP(),
            };
            console_ffi::SetConsoleOutputCP(Self::UTF8);
            console_ffi::SetConsoleCP(Self::UTF8);
            guard
        }
    }
}

#[cfg(windows)]
impl Drop for ConsoleCodePage {
    fn drop(&mut self) {
        // A code page of 0 means there was no console to query; leave it alone.
        // SAFETY: as above.
        unsafe {
            if self.previous_output != 0 {
                console_ffi::SetConsoleOutputCP(self.previous_output);
            }
            if self.previous_input != 0 {
                console_ffi::SetConsoleCP(self.previous_input);
            }
        }
    }
}

/// Consoles on other platforms are UTF-8 already, so there is nothing to guard.
#[cfg(not(windows))]
pub(crate) struct ConsoleCodePage;

/// Turn a POSIX locale such as `de_DE.UTF-8` into a BCP-47 tag (`de-DE`).
///
/// `C` and `POSIX` name no language and yield `None`.
pub(crate) fn posix_locale_tag(value: &str) -> Option<String> {
    let tag = value.split(['.', '@']).next()?.replace('_', "-");
    match tag.as_str() {
        "" | "C" | "POSIX" => None,
        _ => Some(tag),
    }
}

/// The user's language as the operating system reports it.
///
/// The POSIX variables come first, in their own order of precedence, so a
/// language set explicitly for one shell still wins on Windows too.
pub(crate) fn system_locale() -> Option<String> {
    ["LC_ALL", "LC_MESSAGES", "LANG"]
        .iter()
        .filter_map(|var| std::env::var(var).ok())
        .find_map(|value| posix_locale_tag(&value))
        .or_else(windows_user_locale)
}

/// The Windows display language of the current user, e.g. `de-DE`.
#[cfg(windows)]
pub(crate) fn windows_user_locale() -> Option<String> {
    // LOCALE_NAME_MAX_LENGTH.
    let mut buffer = [0u16; 85];
    // SAFETY: the buffer is writable and its length is passed alongside it.
    let written =
        unsafe { console_ffi::GetUserDefaultLocaleName(buffer.as_mut_ptr(), buffer.len() as i32) };
    // The count includes the terminating NUL; 0 means failure.
    let len = usize::try_from(written).ok()?.checked_sub(1)?;
    let name = String::from_utf16(&buffer[..len]).ok()?;
    (!name.is_empty()).then_some(name)
}

#[cfg(not(windows))]
pub(crate) fn windows_user_locale() -> Option<String> {
    None
}

#[cfg(not(windows))]
impl ConsoleCodePage {
    /// No-op counterpart of the Windows implementation.
    pub(crate) fn switch_to_utf8() -> Self {
        Self
    }
}
