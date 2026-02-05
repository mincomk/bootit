use efivar::VarManager;
use serde::{Deserialize, Serialize};

pub type EfiSystem = Box<dyn VarManager>;

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct BootEntryIdentifier {
    pub partition_signature: String,
    pub file_path: String,
}

#[derive(Debug, Clone)]
pub struct UniqueBootEntry {
    pub id: u16,
    pub identifier: BootEntryIdentifier,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootAlias {
    pub name: String,
    pub label: String,
    pub identifier: BootEntryIdentifier,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub aliases: Vec<BootAlias>,
}
