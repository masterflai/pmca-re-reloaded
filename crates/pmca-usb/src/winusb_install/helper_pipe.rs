//! The local channel between the program and its elevated driver helper.
//!
//! A named pipe that only the user's own account may open, with exactly one
//! instance, closed to remote machines, and whose ends check each other's
//! process id: the helper accepts only the process that started it, and that
//! process talks only to the helper it started. Nothing here interprets what
//! is sent; the protocol lives with the caller.

use std::fs::File;
use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle, RawHandle};

use windows::Win32::Foundation::{
    CloseHandle, ERROR_PIPE_CONNECTED, HANDLE, HLOCAL, INVALID_HANDLE_VALUE, LocalFree,
};
use windows::Win32::Security::Authorization::{
    ConvertSidToStringSidW, ConvertStringSecurityDescriptorToSecurityDescriptorW, SDDL_REVISION_1,
};
use windows::Win32::Security::{
    GetTokenInformation, PSECURITY_DESCRIPTOR, SECURITY_ATTRIBUTES, TOKEN_QUERY, TOKEN_USER,
    TokenUser,
};
use windows::Win32::Storage::FileSystem::{FILE_FLAG_FIRST_PIPE_INSTANCE, PIPE_ACCESS_DUPLEX};
use windows::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeW, GetNamedPipeClientProcessId, GetNamedPipeServerProcessId,
    PIPE_READMODE_BYTE, PIPE_REJECT_REMOTE_CLIENTS, PIPE_TYPE_BYTE, PIPE_WAIT,
};
use windows::Win32::System::Threading::{
    GetCurrentProcess, INFINITE, OpenProcess, OpenProcessToken, PROCESS_SYNCHRONIZE,
    WaitForSingleObject,
};
use windows::core::{PCWSTR, PWSTR};

use super::{Error, Result, wide};

/// Full path of a pipe called `name`.
pub fn pipe_path(name: &str) -> String {
    format!(r"\\.\pipe\{name}")
}

/// The security identifier of the account this process runs as, e.g.
/// `S-1-5-21-…`. The helper restricts its pipe to it.
pub fn current_user_sid() -> Result<String> {
    // SAFETY: the token handle is closed below; the buffer is sized by the
    // first call and outlives every pointer into it; the string from
    // ConvertSidToStringSidW is freed with LocalFree as documented.
    unsafe {
        let mut token = HANDLE::default();
        OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token)
            .map_err(|e| Error::Install(format!("token: {e}")))?;
        let mut size = 0u32;
        let _ = GetTokenInformation(token, TokenUser, None, 0, &mut size);
        let mut buffer = vec![0u8; size as usize];
        let read = GetTokenInformation(
            token,
            TokenUser,
            Some(buffer.as_mut_ptr().cast()),
            size,
            &mut size,
        );
        let _ = CloseHandle(token);
        read.map_err(|e| Error::Install(format!("token user: {e}")))?;
        let user = &*(buffer.as_ptr() as *const TOKEN_USER);
        let mut text = PWSTR::null();
        ConvertSidToStringSidW(user.User.Sid, &mut text)
            .map_err(|e| Error::Install(format!("sid: {e}")))?;
        let sid = text.to_string().map_err(|e| Error::Install(format!("sid: {e}")));
        let _ = LocalFree(Some(HLOCAL(text.0.cast())));
        sid
    }
}

/// Create the pipe `name`, open only to `user_sid`, and wait until the
/// process `client_pid` has connected. Any other process is refused.
pub fn serve(name: &str, user_sid: &str, client_pid: u32) -> Result<File> {
    let path = wide(&pipe_path(name));
    // Protected DACL: full access for this one account, nobody else.
    let sddl = wide(&format!("D:P(A;;GA;;;{user_sid})"));

    // SAFETY: the security descriptor is freed with LocalFree once the pipe
    // exists; the pipe handle is owned by the returned File, or closed on
    // every error path through OwnedHandle.
    unsafe {
        let mut descriptor = PSECURITY_DESCRIPTOR::default();
        ConvertStringSecurityDescriptorToSecurityDescriptorW(
            PCWSTR(sddl.as_ptr()),
            SDDL_REVISION_1,
            &mut descriptor,
            None,
        )
        .map_err(|e| Error::Install(format!("pipe security: {e}")))?;
        let attributes = SECURITY_ATTRIBUTES {
            nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: descriptor.0,
            bInheritHandle: false.into(),
        };
        let handle = CreateNamedPipeW(
            PCWSTR(path.as_ptr()),
            PIPE_ACCESS_DUPLEX | FILE_FLAG_FIRST_PIPE_INSTANCE,
            PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT | PIPE_REJECT_REMOTE_CLIENTS,
            1,
            4096,
            4096,
            0,
            Some(&attributes),
        );
        let _ = LocalFree(Some(HLOCAL(descriptor.0)));
        if handle == INVALID_HANDLE_VALUE || handle.is_invalid() {
            return Err(Error::last_os("pipe"));
        }
        let owned = OwnedHandle::from_raw_handle(handle.0 as RawHandle);

        if let Err(e) = ConnectNamedPipe(handle, None)
            && e.code() != ERROR_PIPE_CONNECTED.to_hresult()
        {
            return Err(Error::Install(format!("pipe connect: {e}")));
        }
        let mut pid = 0u32;
        GetNamedPipeClientProcessId(handle, &mut pid)
            .map_err(|e| Error::Install(format!("pipe client: {e}")))?;
        if pid != client_pid {
            return Err(Error::Install(format!("unexpected pipe client {pid}")));
        }
        Ok(File::from(owned))
    }
}

/// The process id at the server end of a connected pipe.
pub fn server_pid(pipe: &File) -> Result<u32> {
    let mut pid = 0u32;
    // SAFETY: the handle belongs to `pipe`, which outlives the call.
    unsafe {
        GetNamedPipeServerProcessId(HANDLE(pipe.as_raw_handle()), &mut pid)
            .map_err(|e| Error::Install(format!("pipe server: {e}")))?;
    }
    Ok(pid)
}

/// Block until the process `pid` has ended. Returns at once when it cannot be
/// opened, which includes the case that it already ended.
pub fn wait_for_exit(pid: u32) {
    // SAFETY: the process handle is waited on and closed exactly once.
    unsafe {
        if let Ok(process) = OpenProcess(PROCESS_SYNCHRONIZE, false, pid) {
            let _ = WaitForSingleObject(process, INFINITE);
            let _ = CloseHandle(process);
        }
    }
}
