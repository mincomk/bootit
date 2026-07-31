use std::path::{Path, PathBuf};

use miette::miette;
use which::which;

pub fn check_privileges() -> miette::Result<()> {
    #[cfg(unix)]
    {
        let euid = unsafe { libc::geteuid() };
        if euid != 0 {
            let hint = sudo_hint();
            if hint.is_empty() {
                return Err(miette!(
                    "This program must be run as root (try: sudo bootit ...)"
                ));
            }
            return Err(miette!(
                help = hint,
                "This program must be run as root (try: sudo bootit ...)",
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

/// Builds a hint for the non-root error message.
///
/// `cargo install` places binaries in `~/.cargo/bin`, which is on the user's
/// PATH but not on sudo's `secure_path`. In that case `sudo bootit ...` fails
/// with "command not found" before the program even runs, so the generic
/// "try: sudo bootit ..." advice in the error message is misleading.
///
/// Returns an empty string when the binary lives in a system directory that
/// sudo can already find, so the error message stays short.
#[cfg(unix)]
fn sudo_hint() -> String {
    let exe = std::env::current_exe().ok();
    let home = std::env::var_os("HOME").map(PathBuf::from);
    match (exe, home) {
        (Some(exe), Some(home)) => sudo_hint_for(&exe, &home),
        _ => String::new(),
    }
}

#[cfg(unix)]
fn sudo_hint_for(exe: &Path, home: &Path) -> String {
    let installed_to_user_dir =
        exe.starts_with(home) || exe.to_string_lossy().contains(".cargo/bin");
    if !installed_to_user_dir {
        return String::new();
    }

    format!(
        "bootit is installed in a user-local directory ({}) which sudo does not search, so \
         `sudo bootit ...` will fail with \"command not found\".\n\
         Run it with the full path:\n    sudo {} ...\n\
         or link it into a system directory once:\n    sudo ln -s {} /usr/local/bin/bootit",
        exe.display(),
        exe.display(),
        exe.display(),
    )
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
        Ok(path)
    } else {
        Err(miette!("Could not find 'it' in PATH. Please install it."))
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn no_hint_when_installed_in_system_dir() {
        let hint = sudo_hint_for(Path::new("/usr/local/bin/bootit"), Path::new("/home/user"));
        assert_eq!(hint, "");
    }

    #[test]
    fn hint_for_cargo_install_under_home() {
        let hint = sudo_hint_for(
            Path::new("/home/user/.cargo/bin/bootit"),
            Path::new("/home/user"),
        );
        assert!(hint.contains("/home/user/.cargo/bin/bootit"));
        assert!(hint.contains("sudo /home/user/.cargo/bin/bootit ..."));
        assert!(hint.contains("ln -s"));
    }

    #[test]
    fn hint_for_cargo_install_under_other_home() {
        // Installed via cargo but for a different user than the one running it.
        let hint = sudo_hint_for(
            Path::new("/home/other/.cargo/bin/bootit"),
            Path::new("/home/user"),
        );
        assert!(hint.contains("ln -s"));
    }
}
