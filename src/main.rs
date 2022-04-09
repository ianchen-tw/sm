use std::{env, path::PathBuf};

use clap::{Parser, Subcommand};
use log::debug;
use simplelog::*;

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
        run_opts.log_level = LevelFilter::Debug;
    }

    if let Some(home) = args.home {
        let localdir: PathBuf = PathBuf::from(home);

        println!("Switch home to: {}", localdir.display());
        if let Err(err) = env::set_current_dir(localdir.as_path()) {
            return Err(format!(
                "Bad home folder: {} -- {}",
                localdir.display(),
                err
            ));
        }
    }

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
