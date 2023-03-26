use std::path::Path;

use config::Config;
use daily_directory::DailyDirectory;

mod config;
mod daily_directory;

const CONFIG_FILE_PATH: &str = "resource/config.toml";
const TODAY_JUNCTION: &str = "today";

fn main() {
    // Load the config file.
    let config = Config::from_file(&CONFIG_FILE_PATH.to_string())
        .expect("Failed to load a config file.");
    println!("config = {:?}", config);

    // Create a directory of today.
    let dir_of_today = DailyDirectory::direcory_of_today(
        Path::new(&config.parent_dir_path),
        &config.date_format);
    println!("dir_of_today = {:?}", dir_of_today);
    dir_of_today.create()
        .expect("Failed to create a directory of today.");

    // Create a junction of today.
    dir_of_today.create_junction(Path::new(TODAY_JUNCTION))
        .expect("Failed to create a junction of today.")
}
