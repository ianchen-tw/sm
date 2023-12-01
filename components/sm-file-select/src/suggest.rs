use anyhow::Result;
use dyn_clone::DynClone;
use log::debug;
use std::{
    fs,
    path::{Component, Path, PathBuf},
    vec,
};

pub trait ListDir: DynClone {
    /// List entries with full path
    fn list_entries(&self, dir: &Path) -> Result<Vec<String>>;
}

dyn_clone::clone_trait_object!(ListDir);

#[derive(Clone)]
pub struct PathSuggester {
    root: PathBuf,
    parents: PathBuf,
    lister: Box<dyn ListDir>,
}

fn extend_path(base: &Path, path: &Path) -> PathBuf {
    let mut parts: Vec<String> = vec![];
    for p in path.components() {
        if let Component::Normal(p) = p {
            parts.push(p.to_string_lossy().to_string())
        }
    }
    let extended_path: PathBuf = parts.iter().fold(base.into(), |acc, layer| acc.join(layer));
    extended_path
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

        if instance.current_path().exists()
            && instance.current_path().is_dir()
            && relative_path.ends_with('/')
        {
            // We only list current directory if the input ends with a slash('/')
        } else {
            instance.parents.pop();
        }
        instance
    }

    pub fn suggest_with_strategy_all_nodes(&self) -> Vec<String> {
        // println!("Listing files in path: {}", &self.get_path().display());
        if let Ok(mut result) = self.lister.list_entries(&self.current_path()) {
            result.sort();
            result
        } else {
            vec![]
        }
    }

    /// Provide a common prefix of all suggested points
    pub fn suggest_common_prefix(&self, input: String) -> String {
        let all_nodes = self.suggest_with_strategy_filter(input);
        find_common_prefix(&all_nodes)
    }

    pub fn suggest_with_strategy_filter(&self, input: String) -> Vec<String> {
        // input_actual.push(input);
        debug!(
            "suggest_with_strategy_filter, root={:#?}, input={:#?}",
            self.root.display(),
            input
        );

        let to_match: PathBuf = extend_path(self.root.as_path(), Path::new(&input));

        let mut result = vec![];
        for filename in self.suggest_with_strategy_all_nodes() {
            if filename.starts_with(&to_match.to_string_lossy().to_string()) {
                result.push(filename);
            }
        }
        result.sort();
        result
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

/// Return the common prefix of all strings
fn find_common_prefix(data: &Vec<String>) -> String {
    let mut ret = String::new();

    if data.is_empty() {
        return ret;
    }

    let mut data = data.clone();
    data.sort();

    let mut first = data.first().unwrap().chars();
    let mut last = data.last().unwrap().chars();

    loop {
        match (first.next(), last.next()) {
            (Some(c1), Some(c2)) if c1 == c2 => {
                ret.push(c1);
            }
            _ => return ret,
        }
    }
}
#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::vec;

    use anyhow::{Ok, Result};

    use super::{find_common_prefix, ListDir, PathSuggester};
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
        fn list_entries(&self, _dir: &Path) -> Result<Vec<String>> {
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
            sg.suggest_with_strategy_all_nodes(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }

    fn make_strings(v: Vec<&str>) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn check_common_prefix() {
        assert_eq!(
            find_common_prefix(&make_strings(vec!["aab", "aac", "aah"])),
            "aa"
        );

        assert_eq!(
            find_common_prefix(&make_strings(vec!["", "aac", "aah"])),
            ""
        );

        assert_eq!(
            find_common_prefix(&make_strings(vec!["bbab", "bbab22", "bbab23"])),
            "bbab"
        );
    }
}
