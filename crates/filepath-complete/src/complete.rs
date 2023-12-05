use crate::lcp;
use crate::suggest::FileSuggest;

use inquire::{
    autocompletion::{Autocomplete, Replacement},
    CustomUserError,
};
use log::debug;

/// TODO: find better location for this function
pub fn get_home() -> String {
    home::home_dir().unwrap().to_str().unwrap().to_string()
}

#[derive(Clone)]
pub struct FilePathCompleter {
    sg: FileSuggest,
}

impl FilePathCompleter {
    pub fn get_dir_prefix(&self) -> String {
        self.sg.current_path().to_string_lossy().to_string()
    }
}

impl Default for FilePathCompleter {
    fn default() -> Self {
        Self {
            sg: FileSuggest::new(&get_home(), ""),
        }
    }
}

impl Autocomplete for FilePathCompleter {
    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        debug!(
            "Start Completion input={:#?}, highlighted={:#?}",
            input, highlighted_suggestion
        );

        if let Some(suggestion) = highlighted_suggestion {
            let result = suggestion
                .strip_prefix(&format!("{}/", get_home()))
                .unwrap();
            debug!("Complete input `{}`-> `{}`", input, result);
            return Ok(Replacement::Some(result.to_string()));
        }

        debug!("No selected item, replace with common prefix of all candidates");
        let prefix =
            lcp::longest_common_prefix(self.sg.suggest_with_strategy_filter(input.to_string()));
        if prefix.is_empty() {
            return Ok(Replacement::None);
        }

        let result = prefix
            .strip_prefix(&format!("{}/", get_home()))
            .unwrap()
            .to_string();
        debug!("Complete input `{}`-> `{}`", input, result);
        Ok(Replacement::Some(result))
    }

    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        debug!("Start suggest, input={:#?}", input);
        self.sg = FileSuggest::new(&get_home(), input);
        Ok(self.sg.suggest_with_strategy_filter(input.to_string()))
    }
}
