use crate::{config, efi};
use miette::{IntoDiagnostic, miette};
use std::path::PathBuf;

pub fn add(config_path: PathBuf, name: &str, id: u16) -> miette::Result<()> {
    let mut config = config::open_config(&config_path)?;

    let system = efivar::system();

    let entry = efi::find_boot_entry_by_id(&system, id)?
        .ok_or_else(|| miette!("Boot entry with specified ID not found"))?;

    let alias = crate::BootAlias {
        name: name.to_string(),
        label: entry.label,
        identifier: entry.identifier,
    };
    config.add_alias(alias)?;

    config.commit()?;

    println!("Alias '{}' added for boot entry ID {}", name, id);

    Ok(())
}

pub fn list(config_path: PathBuf) -> miette::Result<()> {
    let config = config::open_config(&config_path)?;
    let aliases = config.aliases();

    if aliases.is_empty() {
        println!("No aliases defined.");
    } else {
        for alias in aliases {
            println!("Alias: '{}', Label: '{}'", alias.name, alias.label);
        }
    }

    Ok(())
}

pub fn remove(config_path: PathBuf, name: &str) -> miette::Result<()> {
    let mut config = config::open_config(&config_path)?;

    config.remove_alias(name)?;
    config.commit()?;

    println!("Alias '{}' removed.", name);

    Ok(())
}

pub fn clear(config_path: PathBuf, yes: bool) -> miette::Result<()> {
    if !yes {
        let confirmed = inquire::Confirm::new("Confirm clear?")
            .with_default(true)
            .with_help_message("This will remove all defined aliases.")
            .prompt()
            .into_diagnostic()?;

        if !confirmed {
            println!("Operation cancelled.");
            return Ok(());
        }
    }

    let mut config = config::open_config(&config_path)?;

    config.clear_aliases();
    config.commit()?;

    println!("All aliases cleared.");

    Ok(())
}
