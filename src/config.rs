use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MyToml {
    pub name: String,
    pub age: u8,
}

impl MyToml {
    pub fn config_file() -> &'static Path {
        let p = Path::new("./sm/config.toml");
        return p;
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        let t = match toml::from_str::<MyToml>(s) {
            Ok(m) => m,
            Err(err) => return Err(format!("Cannot parse config - {}", err)),
        };
        return Ok(t);
    }
}
