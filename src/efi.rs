use efivar::efi::{Variable, VariableFlags, VariableVendor};
use miette::IntoDiagnostic;

use crate::{BootEntryIdentifier, EfiSystem, UniqueBootEntry};

pub fn scan(system: &EfiSystem) -> miette::Result<Vec<UniqueBootEntry>> {
    Ok(system
        .get_boot_entries()
        .into_diagnostic()?
        .filter_map(|e| e.0.ok())
        .filter_map(|entry| {
            let file_path_list = entry.entry.file_path_list?;

            let partition_signature = file_path_list.hard_drive.partition_sig.to_string();
            let file_path = file_path_list.file_path.path;

            let identifier = BootEntryIdentifier {
                partition_signature,
                file_path,
            };

            Some(UniqueBootEntry {
                id: entry.id,
                identifier,
                label: entry.entry.description,
            })
        })
        .collect::<Vec<_>>())
}

pub fn find_boot_entry_by_identifier(
    system: &EfiSystem,
    ident: &BootEntryIdentifier,
) -> miette::Result<Option<UniqueBootEntry>> {
    Ok(scan(system)?
        .into_iter()
        .find(|entry| &entry.identifier == ident))
}

pub fn find_boot_entry_by_id(
    system: &EfiSystem,
    id: u16,
) -> miette::Result<Option<UniqueBootEntry>> {
    Ok(scan(system)?.into_iter().find(|entry| entry.id == id))
}

pub fn set_bootnext(system: &mut EfiSystem, id: u16) -> miette::Result<()> {
    let variable = Variable::new_with_vendor("BootNext", VariableVendor::Efi);
    let attributes = VariableFlags::NON_VOLATILE
        | VariableFlags::BOOTSERVICE_ACCESS
        | VariableFlags::RUNTIME_ACCESS;

    let id_bytes = id.to_le_bytes();

    system
        .write(&variable, attributes, &id_bytes)
        .into_diagnostic()?;

    println!("BootNext successfully set to {}", id);

    Ok(())
}
