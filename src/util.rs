use miette::miette;

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
