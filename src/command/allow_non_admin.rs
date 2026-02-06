use std::path::PathBuf;

#[allow(unused)]
pub fn allow_non_admin(it_path: Option<PathBuf>) -> miette::Result<()> {
    #[cfg(target_os = "linux")]
    {
        use std::fs;
        use miette::IntoDiagnostic;
        use crate::util;

        let it_path = match it_path {
            Some(path) => path,
            None => util::find_it()?,
        };

        // set the owner to root
        use nix::unistd::{Gid, Uid, chown};
        use std::os::unix::fs::PermissionsExt;
        chown(&it_path, Some(Uid::from_raw(0)), Some(Gid::from_raw(0))).into_diagnostic()?;

        let metadata = fs::metadata(&it_path).into_diagnostic()?;
        let mut permissions = metadata.permissions();
        let mode = permissions.mode();

        // Set the setuid bit and ensure owner has execute permission
        let new_mode = mode | 0o4755;
        permissions.set_mode(new_mode);
        fs::set_permissions(&it_path, permissions).into_diagnostic()?;

        println!(
            "Set setuid bit on '{}', new mode: {:o}",
            it_path.display(),
            new_mode
        );
        println!("Non-admin users can now use it command to boot.");
    }

    #[cfg(target_os = "windows")]
    {
        println!("The setuid mechanism is not available on Windows.");
        println!("Run bootit from an elevated terminal (right-click -> Run as administrator).");
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        println!("allow-non-admin is not supported on this operating system.");
    }

    Ok(())
}
