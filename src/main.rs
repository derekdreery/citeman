//! Manage your literature and citations.

use citeman::{Citeman, Config};
use structopt::StructOpt;

type ResultBoxError<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, StructOpt)]
#[structopt(name = "citeman", about = "An app for managing citations")]
struct Cli {
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    /// Print the computed location of the database.
    #[structopt(name = "print-db-location")]
    PrintDbLocation,
}

fn main() {
    let args = Cli::from_args();
    pretty_env_logger::formatted_timed_builder()
        .filter(None, args.verbose.log_level().to_level_filter())
        .init();
    if let Err(e) = run(args) {
        log::error!("{}", e);
        std::process::exit(1);
    }
}

fn run(args: Cli) -> ResultBoxError<()> {
    let citeman = Citeman::open();
    Ok(match args.cmd {
        Cmd::PrintDbLocation => println!("{}", citeman.config().db_path().display()),
    })
    //citeman::store("test/path.pdf");
}
