#![windows_subsystem = "windows"]

use std::io;
use std::fs;
use std::path::Path;
use std::env::current_exe;
use std::path::PathBuf;

use chrono::{Local, LocalResult, NaiveDate, TimeZone};
use serde_derive::Deserialize;

const CONFIG_FILE: &str = "config.toml";
const OLD_DIR: &str = "old";

/// Config data structure
#[derive(Deserialize)]
struct Config {
    parent_dir: String,
    date_format: String,
    days_to_move: u32,
    days_to_remove: u32,
}

fn main() {
    // Deserialize Config
    let config_str = read_config_file().expect("Could not read config file.");
    let config = toml::from_str::<Config>(&config_str)
        .expect("Could not parse config file.");

    // parent directory path
    let parent_dir = Path::new(&config.parent_dir);

    // Create the today's directory.
    let today_dir_path = create_today_dir(parent_dir, &config.date_format)
        .expect("Could not create today's directory.");

    // Create "today" junction.
    create_today_junction(parent_dir, &today_dir_path)
        .expect("Could not create the \"today\" junction.");

    // Move old directories to "old" directory.
    let dir_names = get_dir_names(parent_dir)
        .expect("Cound not get directories.");
    for dir_name in dir_names {
        if !matches_date_format(&dir_name, &config.date_format) {
            continue;
        }
        let days_elapsed = calc_days_elapsed(&dir_name, &config.date_format)
            .expect("Failed to calc days elapsed.");
        if days_elapsed >= config.days_to_move.into() {
            move_to_old_dir(parent_dir, &dir_name);
        }
    }

    // Remove more old directories in "old" direcotry.
    let dir_names = get_dir_names(parent_dir.join(OLD_DIR).as_path())
        .expect("Cound not get directories.");
    for dir_name in dir_names {
        if !matches_date_format(&dir_name, &config.date_format) {
            continue;
        }
        let days_elapsed = calc_days_elapsed(&dir_name, &config.date_format)
            .expect("Failed to calc days elapsed.");
        if days_elapsed >= config.days_to_remove.into() {
            let _ = fs::remove_dir_all(
                parent_dir.join(OLD_DIR).join(dir_name).as_path());
        }
    }
}

/// Read config file to String.
fn read_config_file() -> io::Result<String> {
    let mut exe_dir = current_exe()?;
    exe_dir.pop();

    let config_file = Path::join(&exe_dir, CONFIG_FILE);
    Ok(fs::read_to_string(config_file)?)
}

/// Create the today's directory.
fn create_today_dir(parent_dir_path: &Path, date_format: &String) -> io::Result<PathBuf> {
    let today = Local::today();
    let today_dir_name = today.format(date_format).to_string();
    let mut path_buf = parent_dir_path.to_path_buf();
    path_buf.push(&today_dir_name);
    if path_buf.exists() {
        Ok(path_buf)
    } else {
        match fs::create_dir(&path_buf) {
            Ok(()) => Ok(path_buf),
            Err(e) => Err(e)
        }
    }
}

/// Create "today" junction.
fn create_today_junction(parent_dir_path: &Path, today_dir_path: &Path) -> io::Result<PathBuf> {
    let mut path_buf = parent_dir_path.to_path_buf();
    path_buf.push("today");
    // Delete the old junction.
    if path_buf.exists() {
        if let Err(e) = fs::remove_dir(&path_buf) {
            return Err(e)
        }
    }
    // Create a junction.
    match junction::create(today_dir_path, &path_buf) {
        Ok(()) => Ok(path_buf),
        Err(e) => Err(e)
    }
}

/// Get names of directries in parent directory.
fn get_dir_names(parent_dir: &Path) -> io::Result<Vec<String>> {
    let read_dir = fs::read_dir(parent_dir)?;
    let mut dir_names = Vec::new();
    for dir_entry in read_dir {
        let dir_entry = dir_entry?;
        let metadata = dir_entry.metadata()?;
        if metadata.is_dir() {
            if let Ok(dir_name) = dir_entry.file_name().into_string() {
                dir_names.push(dir_name);
            }
        }
    }
    Ok(dir_names)
}

/// Return whether directory name matches date format.
fn matches_date_format(dir_name: &String, date_format: &String) -> bool {
    match NaiveDate::parse_from_str(dir_name, date_format) {
        Ok(_) => true,
        Err(_) => false
    }
}

/// Calculate days elapsed of directory.
fn calc_days_elapsed(dir_name: &String, date_format: &String) ->
    Result<i64, String> {
    let date = NaiveDate::parse_from_str(dir_name, date_format)
        .map_err(|_| "Failed to parse dir name.".to_string())?;
    if let LocalResult::Single(local_date) = Local.from_local_date(&date) {
        Ok((Local::today() - local_date).num_days())
    } else {
        Err("Failed to convert date.".to_string())
    }
}

/// Move a directory to "old" directory.
fn move_to_old_dir(parent_dir: &Path, dir_name: &String) {
    let src = parent_dir.join(dir_name);
    let old_dir = parent_dir.join(OLD_DIR);
    let dst = old_dir.join(dir_name);
    if !old_dir.exists() {
        let _ = fs::create_dir(old_dir);
    }
    let _ = fs::rename(src, dst);
}
