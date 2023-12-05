use anyhow::Context;
use std::convert::From;
use std::fs::{self};
use std::path::PathBuf;
use std::str::FromStr;

pub const PATH_CONFIG: &str = "./sm/config.toml";
pub const PATH_CONFIG_BAK: &str = "./sm/config.toml.bak";

pub struct FileUtil {
    pub path: PathBuf,
}

impl From<&str> for FileUtil {
    fn from(s: &str) -> Self {
        FileUtil {
            path: PathBuf::from_str(s).unwrap(),
        }
    }
}

impl FileUtil {
    pub fn delete(self) {
        fs::remove_file(self.path).unwrap();
    }

    /// Write all data into file, file must exists
    pub fn dump(&self, data: &[u8]) {
        fs::write(&self.path, data)
            .with_context(|| format!("Unable to write file : {}", &self.path.display()))
            .unwrap();
    }

    /// Check if file exists
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Create all parent folder
    pub fn create_parent_dir(&self) {
        fs::create_dir_all(self.path.parent().unwrap()).unwrap();
    }
}
