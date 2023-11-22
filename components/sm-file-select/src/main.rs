
mod suggest;



use suggest::{PathSuggester};

use inquire::{
    autocompletion::{Autocomplete, Replacement},
    CustomUserError,
};
use log::{debug, info};
use simplelog::*;


#[derive(Clone)]
struct FilePathCompleter{
    sg: PathSuggester,
}

impl Default for FilePathCompleter {
    fn default() -> Self {
        Self {
            sg: PathSuggester::new("/home/ian", ""),
        }
    }
}

impl Autocomplete for FilePathCompleter{
    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        
        debug!("get_completion start input={:#?}, hilighted={:#?}", input, highlighted_suggestion);
        Ok(Some(input.into()))
    }

    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        debug!("get_suggestions start, input={:#?}", input);

        let sg = PathSuggester::new("/home/ian", input);
        // Ok(sg.suggest_with_strategy_all_nodes().unwrap())
        Ok(sg.suggest_with_strategy_filter(input).unwrap())
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
            let res: Vec<&str> = res.iter().take(15).map(|item|item.strip_prefix(&prefix).unwrap() ).collect();
            info!("result {:#?}", res);
        }
        Err(err) => {
            println!("err: {}", err)
        }
    }
}

fn main() {
    init_logger(LevelFilter::Debug);

    try_auto_complete("/home/ian", "");
    try_auto_complete("/home/ian", "D");
    try_auto_complete("/home/ian", "go/");
    try_auto_complete("/home/ian", "./go/");
    try_auto_complete("/home/ian", "./go/b");
    try_auto_complete("/home/ian", "./.ssh");
    try_auto_complete("~", ".");

    // info!("Interactive:");
    // let ans = Text::new("Path Selected: ~/")
    //     .with_autocomplete(FilePathCompleter::default())
    //     .with_help_message("...")
    //     .prompt();

    // match ans {
    //     Ok(path) => println!("Path: {path}"),
    //     Err(error) => println!("Error with questionnaire, try again later: {error:?}"),
    // }
}
