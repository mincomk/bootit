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
        for alias in &self.config.aliases {
            if alias.name == name {
                return Some(alias);
            }
        }
        None
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
        Err(e) => {
            if let Some(io_err) = e.source().and_then(|s| s.downcast_ref::<std::io::Error>()) {
                if io_err.kind() == std::io::ErrorKind::NotFound {
                    let default_config = Config::default();
                    save_config(&path, &default_config)?;
                    return Ok(default_config);
                }
            }
            Err(e)
        }
    }
}
