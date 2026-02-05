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
    Ok(())
}

#[allow(unused)]
pub fn find_it() -> miette::Result<PathBuf> {
    if let Ok(path) = which("it") {
        return Ok(path);
    } else {
        return Err(miette!("Could not find 'it' in PATH. Please install it."));
    }
}
