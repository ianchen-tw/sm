use anyhow::Result;
use dyn_clone::DynClone;
use std::{fs, path::Path, vec};

pub trait ListDir: DynClone {
    /// List entries with full path
    fn list_entries(&self, dir: &Path) -> Result<Vec<String>>;
}

dyn_clone::clone_trait_object!(ListDir);

#[derive(Clone, Default)]
pub struct OsFileLister;

impl ListDir for OsFileLister {
    fn list_entries(&self, dir: &Path) -> Result<Vec<String>> {
        let mut result = vec![];

        let entries = fs::read_dir(dir)?;
        for entry in entries {
            // Full path
            let path = entry?.path();

            let final_name = match path.is_dir() {
                true => {
                    format!("{}/", path.to_string_lossy())
                }
                false => path.to_string_lossy().into_owned(),
            };
            result.push(final_name);
        }
        Ok(result)
    }
}
