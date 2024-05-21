use std::{fs, io, path::PathBuf};

use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::Map;

pub type Machine = String;
pub type LayoutDesc = String;

/// All layouts for all machines.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub machines: Map<Machine, LayoutDesc>,
}

impl Config {
    /// Loads the current user config from disk.
    pub fn new() -> Result<Self, Error> {
        let proj_dirs =
            ProjectDirs::from("org", "MultisampledNight", "layaway").ok_or(Error::UnknownHome)?;

        let path = proj_dirs.config_dir().join("config.toml");
        let source = fs::read_to_string(&path).map_err(|err| Error::Load { err, path })?;

        let config = toml::from_str(&source)?;

        Ok(config)
    }

    /// Returns the unparsed layout DSL description for this machine,
    /// based on the machine's hostname.
    ///
    /// Returns [`Ok`]`(`[`None`]`)` if the config does not contain a layout for this machine.
    /// Returns [`Err`] if the hostname cannot be determined.
    pub fn machine_layout(&self) -> io::Result<Option<&LayoutDesc>> {
        // listen i'm just tired
        // and don't want to introduce another lengthy error type in this module
        // please let me be or fix it i guess
        let hostname = hostname::get()?;
        let Some(hostname) = hostname.to_str() else {
            return Ok(None);
        };
        Ok(self.machines.get(hostname))
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("directories-next could not determine the home directory")]
    UnknownHome,
    #[error(
        "Could not load config file at `{path}` from disk, maybe it doesn't exist yet?\n{err}"
    )]
    Load { err: io::Error, path: PathBuf },
    #[error("Could not parse config file: {0}")]
    Toml(#[from] toml::de::Error),
}
