#[macro_use]
extern crate diesel_migrations;

mod config;
pub mod document;

pub use config::Config;
use diesel::prelude::*;
use document::NewDocument;
use std::path::PathBuf;

diesel_migrations::embed_migrations!();

pub struct Citeman {
    config: Config,
    web: reqwest::Client,
    conn: SqliteConnection,
}

impl Citeman {
    pub fn open() -> Self {
        let config = Config::new().unwrap();
        let web = reqwest::Client::new();
        // todo handle non-utf8 paths
        log::info!("Opening database at path {}", config.db_path().display());
        let conn = SqliteConnection::establish(config.db_path().to_str().unwrap()).unwrap();
        embedded_migrations::run(&conn);
        Citeman { conn, web, config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn store(&self, new_document: NewDocument) -> Result<(), std::io::Error> {
        Ok(())
    }
}

// Available commands

pub fn store(paper: impl Into<PathBuf>) {
    let new_document = NewDocument::from_stdin_interactive(paper).unwrap();
    println!("{:?}", new_document);
    unimplemented!();
}
