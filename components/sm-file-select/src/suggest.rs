use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct PathSuggester<'a, T: ListDir> {
    root: PathBuf,
    parents: PathBuf,
    lister: &'a T,
}

impl<'a, T: ListDir> PathSuggester<'a, T> {
    pub fn new(root: &str, lister: &'a T) -> PathSuggester<'a, T> {
        Self {
            root: root.into(),
            parents: PathBuf::new(),
            lister,
        }
    }

    pub fn suggest_with_strategy_all_nodes(&self) -> Result<Vec<String>> {
        // println!("Listing files in path: {}", &self.get_path().display());
        self.lister.list_filenames(&self.current_path())
    }

    pub fn current_path(&self) -> PathBuf {
        self.root.join(&self.parents)
    }

    pub fn pop_path(&mut self) -> bool {
        self.parents.pop()
    }

    pub fn push_path(&mut self, dir: &str) {
        self.parents.push(dir);
    }
}

pub trait ListDir {
    fn list_filenames(&self, dir: &Path) -> Result<Vec<String>>;
}

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
    use std::cell::RefCell;
    use std::path::Path;
    use std::vec;

    use anyhow::{Ok, Result};

    use super::{ListDir, PathSuggester};
    struct FakeLister {
        payload: RefCell<Vec<String>>,
    }

    impl FakeLister {
        fn set_payload(&self, res: Vec<String>) {
            self.payload.borrow_mut().clone_from(&res);
        }
        fn new() -> Self {
            Self {
                payload: vec![].into(),
            }
        }
    }
    impl ListDir for FakeLister {
        fn list_filenames(&self, _dir: &Path) -> Result<Vec<String>> {
            Ok(self.payload.borrow().clone())
        }
    }

    #[test]
    fn do_pushpop() {
        let lister = FakeLister::new();

        let mut sg = PathSuggester::new("/1", &lister);
        sg.push_path("2");
        assert_eq!(sg.current_path(), Path::new("/1/2"));
        sg.pop_path();
        assert_eq!(sg.current_path(), Path::new("/1"));
        sg.pop_path();
        assert_eq!(sg.current_path(), Path::new("/1"));

        lister.set_payload(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        sg.current_path();
    }

    #[test]
    fn do_suggest() {
        let lister = FakeLister {
            payload: vec![].into(),
        };
        let sg = PathSuggester::new("/1", &lister);

        lister.set_payload(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        assert_eq!(
            sg.suggest_with_strategy_all_nodes().unwrap(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }
}
