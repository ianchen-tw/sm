mod suggest;

use suggest::PathSuggester;

use inquire::{
    autocompletion::{Autocomplete, Replacement},
    CustomUserError, Text,
};
use log::{debug, info};
use simplelog::*;

fn get_home() -> String {
    home::home_dir().unwrap().to_str().unwrap().to_string()
}
#[derive(Clone)]
struct FilePathCompleter {
    sg: PathSuggester,
}

impl Default for FilePathCompleter {
    fn default() -> Self {
        Self {
            sg: PathSuggester::new(&get_home(), ""),
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
            "get_completion start input={:#?}, hilighted={:#?}",
            input, highlighted_suggestion
        );

        if let Some(suggestion) = highlighted_suggestion {
            let result = suggestion
                .strip_prefix(&self.sg.current_path().to_string_lossy().to_string())
                .unwrap();
            return Ok(Replacement::Some(result.to_string()));
        }
        Ok(Replacement::Some(
            self.sg
                .suggest_common_prefix(input.to_string())
                .strip_prefix(&self.sg.current_path().to_string_lossy().to_string())
                .unwrap()
                .to_string(),
        ))
    }

    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        debug!("get_suggestions start, input={:#?}", input);
        let sg = PathSuggester::new(&get_home(), input);
        Ok(sg.suggest_with_strategy_filter(input.to_string()))
    }
}

fn init_logger(level: LevelFilter) {
    TermLogger::init(
        level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
}

fn try_auto_complete(root: &str, input: &str) {
    let mut sg = FilePathCompleter::default();
    info!("try_auto_complete:  root=\"{}\", input=\"{}\"", root, input);

    match sg.get_suggestions(input) {
        Ok(res) => {
            let prefix: String = sg.sg.current_path().to_string_lossy().to_string();
            // todo: remove entry limit
            let res: Vec<&str> = res
                .iter()
                .take(15)
                .map(|item| item.strip_prefix(&prefix).unwrap())
                .collect();
            info!("result {:#?}", res);
        }
        Err(err) => {
            println!("err: {}", err)
        }
    }
}

fn main() {
    init_logger(LevelFilter::Info);

    try_auto_complete("/home/ian/", "");
    try_auto_complete("/home/ian/", "D");
    try_auto_complete("/home/ian/", "go");
    try_auto_complete("/home/ian/", "go/");
    try_auto_complete("/home/ian/", "./go/");
    try_auto_complete("/home/ian/", "./go/b");
    try_auto_complete("/home/ian/", "./.ssh");
    try_auto_complete("~", ".");

    info!("Interactive:");
    let ans = Text::new(&format!("Path Selected: {}/", get_home()))
        .with_autocomplete(FilePathCompleter::default())
        .with_help_message("...")
        .prompt();

    match ans {
        Ok(path) => println!("Path: {path}"),
        Err(error) => println!("Error with questionnaire, try again later: {error:?}"),
    }
}
