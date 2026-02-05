use std::path::PathBuf;

use crate::config;

pub fn scan(config_path: PathBuf) -> miette::Result<()> {
    let system = efivar::system();
    let entries = crate::efi::scan(&system)?;

    let config = config::open_config(&config_path)?;

    if entries.is_empty() {
        println!("No boot entries found.");
    } else {
        println!("Available Boot Entries:");
        for entry in entries {
            let alias_name = config
                .find_alias_by_identifier(&entry.identifier)?
                .map(|alias| alias.name.as_str());

            if let Some(name) = alias_name {
                println!(
                    "ID: {:04X}, Label: {} (Alias: '{}')",
                    entry.id, entry.label, name
                );
            } else {
                println!("ID: {:04X}, Label: {}", entry.id, entry.label);
            }
        }
    }

    Ok(())
}
