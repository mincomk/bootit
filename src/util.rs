use std::path::PathBuf;

use miette::miette;
use which::which;

pub fn check_privileges() -> miette::Result<()> {
    #[cfg(unix)]
    {
        let euid = unsafe { libc::geteuid() };
        if euid != 0 {
            return Err(miette!(
                "This program must be run as root (try: sudo bootit ...)"
            ));
        }
    }

    #[cfg(windows)]
    {
        if !is_elevated() {
            return Err(miette!(
                "This program must be run as Administrator (try: run terminal as administrator)"
            ));
        }
    }

    Ok(())
}

#[cfg(windows)]
fn is_elevated() -> bool {
    use std::ptr::null_mut;
    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
    use windows_sys::Win32::Security::{
        GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY,
    };
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    unsafe {
        let mut token_handle: HANDLE = null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle) == 0 {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut return_length: u32 = 0;
        let size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;

        let result = GetTokenInformation(
            token_handle,
            TokenElevation,
            &mut elevation as *mut _ as *mut _,
            size,
            &mut return_length,
        );

        CloseHandle(token_handle);

        result != 0 && elevation.TokenIsElevated != 0
    }
}

#[allow(unused)]
pub fn find_it() -> miette::Result<PathBuf> {
    if let Ok(path) = which("it") {
        return Ok(path);
    } else {
        return Err(miette!("Could not find 'it' in PATH. Please install it."));
    }
}
