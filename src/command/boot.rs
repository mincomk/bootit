use miette::{IntoDiagnostic, miette};
use std::path::PathBuf;

use crate::{config, efi};

enum BootTarget {
    Id(u16),
    Alias(String),
}

fn parse_boot_target(target: String) -> BootTarget {
    if let Ok(id) = target.parse::<u16>() {
        BootTarget::Id(id)
    } else {
        BootTarget::Alias(target)
    }
}

pub fn boot(config_path: PathBuf, target: String, no_reboot: bool) -> miette::Result<()> {
    let mut system = efivar::system();
    let boot_target = parse_boot_target(target);

    match boot_target {
        BootTarget::Id(id) => {
            println!("Setting BootNext to ID {}", id);

            efi::find_boot_entry_by_id(&system, id)?
                .ok_or_else(|| miette!("Boot entry not found"))?;

            efi::set_bootnext(&mut system, id)?;
        }
        BootTarget::Alias(alias) => {
            println!("Resolving alias '{}'", alias);
            let config = config::open_config(config_path)?;

            if let Some(ref alias) = config.find_alias(&alias) {
                let entry = efi::find_boot_entry_by_identifier(&system, &alias.identifier)?
                    .ok_or_else(|| miette!("Boot entry not found"))?;
                println!(
                    "Setting BootNext to ID {} (alias '{}')",
                    entry.id, alias.name
                );
                efi::set_bootnext(&mut system, entry.id)?;
            } else {
                return Err(miette!("Alias not found"));
            }
        }
    }

    if !no_reboot {
        system_shutdown::reboot().into_diagnostic()?;
    }

    Ok(())
}
