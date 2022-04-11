use std::{fs, path::Path, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SMConfig {
    pub name: String,
    pub age: u8,
}

impl Default for SMConfig {
    fn default() -> Self {
        SMConfig {
            name: String::from_str("Ian").unwrap(),
            age: 18,
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

    pub fn create_file(self) -> Result<(), String> {
        let file_path = SMConfig::config_file();
        if !file_path.exists() {
            fs::create_dir_all(file_path.parent().unwrap()).unwrap();
            let str = toml::to_string(&self).unwrap();
            fs::write(file_path, str.as_bytes()).unwrap();
            Ok(())
        } else {
            Err(String::from_str("File alreay exists").unwrap())
        }
    }
}
