use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectConfig {
    pub name: String,
    pub desc: String,
    pub user: String,
    pub server_addr: String,
    pub port: u32,
    pub pem_path: String,
    pub auth: String,
}

impl Default for ConnectConfig {
    fn default() -> Self {
        ConnectConfig {
            name: "ian".to_string(),
            desc: "desc".to_string(),
            user: "yac".to_string(),
            server_addr: "192.168.1.1".to_string(),
            port: 22,
            pem_path: "~/.ssh/good".to_string(),
            auth: "pem".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SMConfig {
    pub connections: Vec<ConnectConfig>,
}

impl Default for SMConfig {
    fn default() -> Self {
        SMConfig {
            connections: vec![ConnectConfig::default(), ConnectConfig::default()],
        }
    }
}

impl SMConfig {
    pub fn config_file() -> &'static Path {
        let p = Path::new("./sm/config.toml");
        return p;
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        let t = match toml::from_str::<SMConfig>(s) {
            Ok(m) => m,
            Err(err) => return Err(format!("Cannot parse config - {}", err)),
        };
        return Ok(t);
    }

    pub fn create_file(self) -> Result<()> {
        let file_path = SMConfig::config_file();
        fs::create_dir_all(file_path.parent().unwrap()).unwrap();

        let str = toml::to_string(&self).with_context(|| "Cannot convert to toml")?;
        fs::write(file_path, str.as_bytes())
            .with_context(|| format!("Unable to write file : {}", file_path.display()))?;
        Ok(())
    }
}
