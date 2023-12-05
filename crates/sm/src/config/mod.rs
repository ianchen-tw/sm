use anyhow::{Context, Result};
use inquire::Select;
use serde::{Deserialize, Serialize};
use std::fs::{self};

mod auth;
mod connect;
mod file;

pub use auth::AuthMethod;
pub use connect::ConnectConfig;
pub use file::{FileUtil, PATH_CONFIG, PATH_CONFIG_BAK};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SMConfig {
    pub connections: Vec<ConnectConfig>,
}

impl SMConfig {
    // Save current config into file
    pub fn save_config(self) {
        let str = toml::to_string(&self)
            .with_context(|| "Cannot convert config to toml")
            .unwrap();

        let target = FileUtil::from(PATH_CONFIG);
        let backup = FileUtil::from(PATH_CONFIG_BAK);

        if !target.exists() {
            target.create_parent_dir();
            target.dump(str.as_bytes());
            return;
        }

        backup.create_parent_dir();
        backup.dump(str.as_bytes());

        fs::copy(&backup.path, &target.path).unwrap();
        backup.delete();
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        let t = match toml::from_str::<SMConfig>(s) {
            Ok(m) => m,
            Err(err) => return Err(format!("Cannot parse config - {}", err)),
        };
        Ok(t)
    }

    /// Return the index of connect config selected
    pub fn select(&self) -> usize {
        let names = self
            .connections
            .iter()
            .map(|c| String::from(&c.name))
            .collect();
        let result = Select::new("Select a connection", names).prompt().unwrap();
        return self
            .connections
            .iter()
            .position(|config| result == config.name)
            .unwrap();
    }
}
