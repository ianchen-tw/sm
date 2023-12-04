use filepath_complete::FilePathCompleter;

use home::home_dir;
use inquire::Text;
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

fn main() {
    init_logger(LevelFilter::Info);
    println!("Pick a file:");
    let home_folder = home_dir().unwrap().to_str().unwrap().to_string();
    let ans = Text::new(&format!("Path Selected: {}/", home_folder))
        .with_autocomplete(FilePathCompleter::default())
        .with_help_message("...")
        .prompt();

    match ans {
        Ok(path) =>{

            let mut result = home_dir().unwrap();
            result.push(path);
            println!("Path: {}", result.display())
        } ,
        Err(error) => println!("Error with questionnaire, try again later: {error:?}"),
    }
}
