//! Config file module
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use serde::Deserialize;

/// Config data
#[derive(Debug)]
#[derive(Deserialize)]
pub struct Config {
    /// The directory path of the daily directories' parent.
    pub parent_dir_path: String,

    /// The date format of daily directories.
    pub date_format: String,

    /// The number of days until move a daily directory to "old" directory.
    /// 
    /// 0 : Don't move a daily directory.
    pub days_to_move: u32,

    /// The number of days until remove a daily directory.
    /// 
    /// 0 : Don't remove a daily directory.
    pub days_to_remove: u32,
}

impl Config {

    /// Create Config from a config toml file.
    /// 
    /// # Params:
    /// file_path : A file path of a config toml file.
    pub fn from_file(file_path: &String) -> Result<Config, Box<dyn Error>> {
        let mut f = File::open(file_path)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}

