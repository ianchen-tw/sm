mod complete;
mod lcp;
mod listdir;
mod suggest;

use crate::complete::{get_home, FilePathCompleter};

use inquire::autocompletion::Autocomplete;
use inquire::Text;
use log::info;
use simplelog::*;

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
    let mut fp = FilePathCompleter::default();
    info!("try_auto_complete:  root=\"{}\", input=\"{}\"", root, input);

    match fp.get_suggestions(input) {
        Ok(res) => {
            let prefix: String = fp.get_dir_prefix();
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
