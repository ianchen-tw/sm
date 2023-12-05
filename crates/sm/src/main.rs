use std::{
    env::{self},
    fs,
    path::PathBuf,
    str::FromStr,
};

use anyhow::Result;
use clap::{Parser, Subcommand};
use dirs::home_dir;
use log::{debug, info};
use simplelog::*;

use crate::config::{SMConfig, PATH_CONFIG};

mod ask;
mod cmds;
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
    ask::init();

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

    let file_path = PathBuf::from_str(PATH_CONFIG).unwrap();

    let sm_config = if !file_path.exists() {
        if let Command::CmdConnect = run_opts.run_command {
            info!("File not exists, use `config` to create a config file first.");
            info!("Use -h option for help message.");
            std::process::exit(0)
        }
        SMConfig::default()
    } else {
        SMConfig::parse(fs::read_to_string(file_path).unwrap().as_str()).unwrap()
    };

    debug!("Load config !, {:?}", sm_config);

    match run_opts.run_command {
        Command::CmdConfig => {
            cmds::do_cmd_config(sm_config);
        }
        Command::CmdConnect => {
            cmds::do_connect_subcmd(sm_config);
        }
    }
    debug!("Program end");
}

fn parse_args(args: Args) -> Result<RunOpts, String> {
    let mut run_opts: RunOpts = RunOpts::default();

    if args.debug {
        run_opts.set_debug_level(LevelFilter::Debug);
    }

    if let Some(cmd) = args.command {
        run_opts.run_command = cmd;
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
