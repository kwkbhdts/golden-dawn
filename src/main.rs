use std::fs;
use std::path::Path;

use config::Config;
use daily_directory::DailyDirectory;

mod debug;
mod config;
mod daily_directory;


const CONFIG_FILE_PATH: &str = "resource/config.toml";
const TODAY_JUNCTION: &str = "today";
const OLD_DIR_NAME: &str = "old";

fn main() {
    // Load the config file.
    let config = Config::from_file(&CONFIG_FILE_PATH.to_string())
        .expect("Failed to load a config file.");
    debug!(println!("config = {:?}", config));

    // Create a directory of today.
    let parent_dir_path = Path::new(&config.parent_dir_path);
    let date_format = &config.date_format;
    let dir_of_today = DailyDirectory::direcory_of_today(parent_dir_path, date_format);
    debug!(println!("dir_of_today = {:?}", dir_of_today));
    dir_of_today.create()
        .expect("Failed to create a directory of today.");

    // Create a junction of today.
    dir_of_today.create_junction(Path::new(TODAY_JUNCTION))
        .expect("Failed to create a junction of today.");

    // Get daily directories.
    let daily_directories =
        DailyDirectory::search_daily_directories(parent_dir_path, date_format)
            .expect("Failed to search daily directoris.");
    debug!(println!("daily_directories = {:?}", daily_directories));

    // Move old directories.
    let old_dir_path = parent_dir_path.join(OLD_DIR_NAME);
    for x in daily_directories {
        if x.is_old(config.days_to_move) {
            fs::create_dir_all(&old_dir_path)
                .unwrap_or_else(|_| panic!("Failed to create \"{}\" directory.", OLD_DIR_NAME));
            x.move_to(&old_dir_path)
                .expect("Failed to move a daily directory.")
        }
    }

    // Get old daily directories.
    let old_daily_directories =
        DailyDirectory::search_daily_directories(&old_dir_path, date_format)
            .expect("Failed to search old daily directoris.");
    debug!(println!("old_daily_directories = {:?}", old_daily_directories));

    // Remove more older directories.
    for x in old_daily_directories {
        if x.is_old(config.days_to_remove) {
            x.remove().expect("Failed to remove a old daily directory.");
        }
    }

}
