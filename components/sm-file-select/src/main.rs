
mod suggest;

use std::fs::File;

use suggest::{PathSuggester};

use inquire::{
    autocompletion::{Autocomplete, Replacement},
    CustomUserError, Text,
};
use log::{debug, info};
use simplelog::*;


fn try_list(root: &str, relative_path: &str) {


    let sg = PathSuggester::new(root, relative_path);

    match sg.suggest_with_strategy_all_nodes() {
        Ok(res) => {
            print!("suggest under `{}`: [", sg.current_path().display());
            let total = 3;
            for (i, pick) in res.iter().take(total).enumerate() {
                print!("{}, ", pick);
                if i == total - 1 {
                    print!("...")
                }
            }
            println!("]");
        }
        Err(err) => {
            println!("err: {}", err)
        }
    }
}

#[derive(Clone)]
struct FilePathCompleter{
    times: u128,
    sg: PathSuggester,
}

impl Default for FilePathCompleter {
    fn default() -> Self {
        Self {
            times:0,
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
        

        debug!("\nget-complete: {:#?}, {:#?}\n", input, highlighted_suggestion);
        Ok(Some(input.into()))
    }

    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        debug!("\nget-suggest: {:#?}\n", input);
        
        let ans = vec![
            format!("{} 1|{}",input, self.times.saturating_add(1)),
            format!("{} 2|{}",input, self.times.saturating_add(1)),
            format!("{} 3|{}",input, self.times.saturating_add(1)),
        ];
        self.times = self.times.saturating_add(1);
        Ok(ans)
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

fn main() {
    init_logger(LevelFilter::Debug);

    println!("select file");

    try_list("/home/ian", "");
    try_list("/home/ian", "./go");
    try_list("/home/ian", "./.ssh");
    try_list("/", "home/ian/g");
    //     try_list("~", ".");

    info!("Interactive:");
    let ans = Text::new("Path Selected: ~/")
        .with_autocomplete(FilePathCompleter::default())
        .with_help_message("...")
        .prompt();

    match ans {
        Ok(path) => println!("Path: {path}"),
        Err(error) => println!("Error with questionnaire, try again later: {error:?}"),
    }
}
