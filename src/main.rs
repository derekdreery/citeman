//! Manage your literature and citations.

use citeman::{Citeman, Config};
use failure::Fail;
use std::{
    ffi::OsStr,
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
    process::Command,
};
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
    /// Create the database, and setup diesel to find it.
    #[structopt(name = "setup")]
    Setup {
        /// If true, any old database will be deleted. WARNING use with caution!
        #[structopt(long = "force")]
        force: bool,
    },
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
        Cmd::Setup { force } => setup_db(&citeman, force)?,
    })
    //citeman::store("test/path.pdf");
}

fn setup_db(citeman: &Citeman, force: bool) -> ResultBoxError<()> {
    let database_path = citeman.config().db_path();
    let diesel_cmd = executable_location("diesel")?;
    // create .env

    // remove existing database
    if database_path.exists() {
        if force {
            log::warn!("database already exists - deleting it and recreating");
            fs::remove_file(&database_path)?;
        } else {
            return Err("database already exists".into());
        }
    }

    //let mut f = File::create("./.env")?;
    //write!(f, "DATABASE_URL={}", citeman.config().db_path().display())?;
    Ok(())
}

fn run_diesel_setup(config: &Config) -> ResultBoxError<()> {
    let diesel_cmd = executable_location("diesel")?;
    let output = Command::new(&diesel_cmd)
        .arg("--database-url")
        .arg(config.db_path())
        .arg("--config-file")
        .arg(config.diesel_config_path())
        .current_dir(config.data_directory())
        .output()?;
}

/// Get the location of an executable.
///
/// Basically a thin wrapper round `which::which` to add some logging.
fn executable_location(name: impl AsRef<OsStr>) -> ResultBoxError<PathBuf> {
    let name = name.as_ref();
    let path = which::which(name).map_err(|e| e.compat())?;
    log::trace!("{} location: {}", name.to_string_lossy(), path.display());
    Ok(path)
}
