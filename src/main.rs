use std::{
    env::{self},
    fs,
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use dirs::home_dir;
use log::{debug, info};
use simplelog::*;

use crate::config::SMConfig;

mod config;

/// SSh Manager
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Debug mode
    #[clap(short, long)]
    debug: bool,

    /// Custom home folder
    #[clap(long)]
    home: Option<PathBuf>,

    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    ///Connect to remot hosts
    #[clap(name = "connect")]
    CmdConnect,
    ///Config settings
    #[clap(name = "config")]
    CmdConfig,
}

struct RunOpts {
    log_level: LevelFilter,
    run_command: Command,
}
impl Default for RunOpts {
    fn default() -> Self {
        Self {
            log_level: LevelFilter::Info,
            run_command: Command::CmdConnect,
        }
    }
}

impl RunOpts {
    fn set_debug_level(&mut self, level: LevelFilter) -> &Self {
        self.log_level = level;
        return self;
    }
}

fn main() {
    let args = Args::parse();

    let run_opts: RunOpts = match parse_args(args) {
        Ok(opts) => opts,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(255);
        }
    };

    init_logger(run_opts.log_level);
    debug!("Program Start");

    let file_path = config::SMConfig::config_file();

    if !file_path.exists() {
        let config = config::SMConfig::default();
        config.create_file().unwrap();
        info!("File not exists, create a default config");
        std::process::exit(0);
    }

    // Config file exists
    let sm_config = SMConfig::parse(fs::read_to_string(file_path).unwrap().as_str()).unwrap();

    println!("Name : {}", sm_config.name);


    // let config_file = MyToml::parse(s)
    debug!("Load config !");

    match run_opts.run_command {
        Command::CmdConfig => {
            println!("Cmd Configs")
        }
        Command::CmdConnect => {
            println!("Cmd Connect")
        }
    }
    debug!("Program end");
}

fn parse_args(args: Args) -> Result<RunOpts, String> {
    let mut run_opts: RunOpts = RunOpts::default();

    if args.debug {
        run_opts.set_debug_level(LevelFilter::Debug);
    }

    let dir = match args.home {
        Some(custom_home) => custom_home,
        None => home_dir().unwrap(),
    };

    if let Err(err) = env::set_current_dir(dir.as_path()) {
        return Err(format!("Bad home folder: {} -- {}", dir.display(), err));
    }
    debug!("Switch home to: {}", dir.display());
    return Ok(run_opts);
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
