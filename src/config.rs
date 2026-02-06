use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use miette::IntoDiagnostic;

use crate::{BootEntryIdentifier, Config};

pub struct ConfigHandle {
    path: PathBuf,
    config: Config,
}

impl ConfigHandle {
    pub fn find_alias<'a>(&'a self, name: &str) -> Option<&'a crate::BootAlias> {
        self.config.aliases.iter().find(|&alias| alias.name == name).map(|v| v as _)
    }

    pub fn aliases(&self) -> &Vec<crate::BootAlias> {
        &self.config.aliases
    }

    pub fn remove_alias(&mut self, name: &str) -> miette::Result<()> {
        let initial_len = self.config.aliases.len();
        self.config.aliases.retain(|alias| alias.name != name);

        if self.config.aliases.len() == initial_len {
            return Err(miette::miette!("Alias '{}' not found", name));
        }
        Ok(())
    }

    pub fn find_alias_by_identifier<'a>(
        &'a self,
        identifier: &BootEntryIdentifier,
    ) -> miette::Result<Option<&'a crate::BootAlias>> {
        for alias in &self.config.aliases {
            if &alias.identifier == identifier {
                return Ok(Some(alias));
            }
        }
        Ok(None)
    }

    pub fn clear_aliases(&mut self) {
        self.config.aliases.clear();
    }

    pub fn has_alias(&self, name: &str) -> bool {
        self.find_alias(name).is_some()
    }

    pub fn add_alias(&mut self, alias: crate::BootAlias) -> miette::Result<()> {
        if self.has_alias(&alias.name) {
            return Err(miette::miette!(
                "Alias name '{}' already exists",
                alias.name
            ));
        }
        self.config.aliases.push(alias);
        Ok(())
    }

    pub fn commit(self) -> miette::Result<()> {
        save_config(&self.path, &self.config)?;
        Ok(())
    }
}

pub fn open_config(path: impl AsRef<Path>) -> miette::Result<ConfigHandle> {
    let config = load_config(&path)?;
    Ok(ConfigHandle {
        path: path.as_ref().to_path_buf(),
        config,
    })
}

fn save_config(path: impl AsRef<Path>, config: &Config) -> miette::Result<()> {
    let yaml_string = serde_yaml::to_string(config).into_diagnostic()?;

    let mut file = File::create(path).into_diagnostic()?;
    file.write_all(yaml_string.as_bytes()).into_diagnostic()?;
    file.sync_all().into_diagnostic()?;

    Ok(())
}

fn load_config_raw(path: impl AsRef<Path>) -> miette::Result<Config> {
    let yaml_string = fs::read_to_string(path).into_diagnostic()?;
    let config: Config = serde_yaml::from_str(&yaml_string).into_diagnostic()?;
    Ok(config)
}

fn load_config(path: impl AsRef<Path>) -> miette::Result<Config> {
    match load_config_raw(&path) {
        Ok(config) => Ok(config),
        Err(_) => {
            let default_config = Config::default();
            save_config(&path, &default_config)?;
            Ok(default_config)
        }
    }
}

pub fn determine_config_path(path: Option<PathBuf>) -> miette::Result<PathBuf> {
    let config_path = match path {
        Some(p) => p,
        None => default_config_path(),
    };

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).into_diagnostic()?;
    }

    if !config_path.exists() {
        let default_config = Config::default();
        save_config(&config_path, &default_config)?;
    }

    Ok(config_path)
}

fn default_config_path() -> PathBuf {
    // HEURISTIC: BOOTIT_CONFIG_PATH, /etc/bootit.yaml,
    // C:\ProgramData\bootit\config.yaml

    if let Ok(env_path) = std::env::var("BOOTIT_CONFIG_PATH") {
        return PathBuf::from(env_path);
    }

    if cfg!(target_os = "windows") {
        let program_data =
            std::env::var("PROGRAMDATA").unwrap_or_else(|_| "C:\\ProgramData".to_string());
        PathBuf::from(format!("{}\\bootit\\config.yaml", program_data))
    } else {
        PathBuf::from("/etc/bootit.yaml".to_string())
    }
}
