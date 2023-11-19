use anyhow::Result;
use dyn_clone::DynClone;
use std::{
    fs,
    path::{Component, Path, PathBuf},
};

pub trait ListDir: DynClone {
    fn list_filenames(&self, dir: &Path) -> Result<Vec<String>>;
}

dyn_clone::clone_trait_object!(ListDir);

#[derive(Clone)]
pub struct PathSuggester {
    root: PathBuf,
    parents: PathBuf,
    lister: Box<dyn ListDir>,
}

impl PathSuggester {
    pub fn new(root: &str, relative_path: &str) -> PathSuggester {
        Self::build(root, relative_path, Box::new(OsFileLister))
    }

    pub fn new_with_lister(root: &str, lister: Box<dyn ListDir>) -> PathSuggester {
        Self {
            root: root.into(),
            parents: PathBuf::new(),
            lister,
        }
    }

    fn build(root: &str, relative_path: &str, lister: Box<dyn ListDir>) -> PathSuggester {
        let mut instance = Self::new_with_lister(root, lister);
        for comp in Path::new(relative_path).components() {
            if let Component::Normal(part) = comp {
                instance.push_path(part.to_str().unwrap());
            }
        }

        if !instance.current_path().exists() {
            instance.parents.pop();
        }
        instance
    }

    pub fn suggest_with_strategy_all_nodes(&self) -> Result<Vec<String>> {
        // println!("Listing files in path: {}", &self.get_path().display());
        self.lister.list_filenames(&self.current_path())
    }

    pub fn current_path(&self) -> PathBuf {
        self.root.join(&self.parents)
    }

    pub fn push_path(&mut self, dir: &str) {
        self.parents.push(dir);
    }
}

#[derive(Clone, Default)]
pub struct OsFileLister;

impl ListDir for OsFileLister {
    fn list_filenames(&self, dir: &Path) -> Result<Vec<String>> {
        let mut result = vec![];

        let entries = fs::read_dir(dir)?;
        for entry in entries {
            let entry = entry?;
            if let Some(name) = entry.path().file_name() {
                result.push(name.to_string_lossy().into_owned());
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::vec;

    use anyhow::{Ok, Result};

    use super::{ListDir, PathSuggester};
    #[derive(Clone, Default)]
    struct FakeLister {
        payload: Vec<String>,
    }

    impl FakeLister {
        fn new(payload: Vec<String>) -> Self {
            Self { payload }
        }
    }
    impl ListDir for FakeLister {
        fn list_filenames(&self, _dir: &Path) -> Result<Vec<String>> {
            Ok(self.payload.clone())
        }
    }

    #[test]
    fn do_pushpop() {
        let lister = Box::new(FakeLister::new(vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
        ]));

        let mut sg = PathSuggester::new_with_lister("/1", lister);
        sg.push_path("2");
        assert_eq!(sg.current_path(), Path::new("/1/2"));
        sg.current_path();
    }

    #[test]
    fn do_suggest() {
        let lister = Box::new(FakeLister::new(vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
        ]));
        let sg = PathSuggester::new_with_lister("/1", lister);
        assert_eq!(
            sg.suggest_with_strategy_all_nodes().unwrap(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }
}
