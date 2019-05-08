use directories::ProjectDirs;
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use uuid::Uuid;

pub struct Config {
    folders: ProjectDirs,
    db_path: PathBuf,
    papers_dir: PathBuf,
    diesel_config_path: PathBuf,
}

impl Config {
    /// Get the config from the system.
    pub fn new() -> io::Result<Config> {
        let folders =
            ProjectDirs::from("", "citeman", "citeman").expect("Cannot get home directory");
        let data_dir = folders.data_dir();
        let db_path = data_dir.join("db.sqlite");
        let papers_dir = data_dir.join("papers");
        let diesel_config_path = data_dir.join("diesel.toml");
        // make sure we have all the dirs we need
        fs::create_dir_all(&papers_dir)?;
        Ok(Config {
            folders,
            db_path,
            papers_dir,
            diesel_config_path,
        })
    }

    /// The folder that all app data goes in.;
    pub fn data_directory(&self) -> &Path {
        self.folders.data_dir()
    }

    /// Get the location that papers will be stored in
    pub fn papers_directory(&self) -> &Path {
        self.papers_dir.as_ref()
    }

    /// Get the location of a specific paper
    pub fn paper_path(&self, paper_id: Uuid) -> PathBuf {
        self.papers_directory()
            .join(format!("papers/{}", paper_id.to_hyphenated()))
    }

    /// Get the location of the database, cache for future use
    pub fn db_path(&self) -> &Path {
        self.db_path.as_ref()
    }

    /// Get the location of the database, cache for future use
    pub fn diesel_config_path(&self) -> &Path {
        self.diesel_config_path.as_ref()
    }
}
