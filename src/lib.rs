//! golden-dawn : Daily Directory Maker

// Standard libraries ------------------
use std::fs::remove_file;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
// Cargo libraries ----------------------
use chrono::{Datelike, Local, NaiveDate};
// Modules ------------------------------
mod util;
// ---------------------------------------

/// The main function of golden-dawn.
pub fn main() {
    // Init logger
    env_logger::init();

    // Get parant dir path
    let parent_dir_path = match get_parent_dir_path() {
        Ok(path) => path,
        Err(e) => {
            log::error!("{}", e);
            return;
        }
    };

    // Create today directory
    let today_dir_path = match create_today_dir(&parent_dir_path) {
        Ok(path) => path,
        Err(e) => {
            log::error!("{}", e);
            return;
        }
    };

    // Update "today" symbolic link
    match update_today_symbolic_link(&today_dir_path) {
        Ok(_) => (),
        Err(e) => {
            log::error!("{}", e);
            return;
        }
    }
}

/// It converts a NaiveDate to a direcory name string.
fn get_dir_name_of(date: NaiveDate) -> String {
    return format!("{}-{:02}-{:02}", date.year(), date.month(), date.day());
}

/// Get parent dir path
///
/// # Returns
///
/// It return parent dir path of daily directories and "today" symbolic link.
///
/// # TODO
///
/// * It should get the path from a setting file.
///
fn get_parent_dir_path() -> Result<PathBuf, String> {
    let current_dir = Path::new(".").to_path_buf();
    return Ok(current_dir);
}

/// Create a directory of today
///
/// # Returns
///
/// It returns PathBuf of created directory if succeeded.
/// It returns error message string if failed.
///
fn create_today_dir(parent_dir_path: &PathBuf) -> Result<PathBuf, String> {
    log::debug!("create_today_dir start");

    // Get target directory path
    let today = Local::now().date_naive();
    let mut target_dir_path = util::get_dir_path_of_exe(parent_dir_path);
    target_dir_path.push(get_dir_name_of(today));

    // Log target direcotry path
    let target_dir_path_str = match target_dir_path.to_str() {
        Some(str) => str,
        None => return Err("Couldn't get the target directory path.".to_string()),
    };
    log::info!("target_path = {}", target_dir_path_str);

    // Create target directory
    return match util::create_dir(&target_dir_path) {
        Ok(_) => Ok(target_dir_path),
        Err(e) => {
            let error_message = format!(
                "Couldn't create the directory. ({}) : {}",
                target_dir_path.to_str().unwrap(),
                e.to_string()
            );
            Err(error_message)
        }
    };
}

//. Get "today" symbolic link path
fn get_today_symbolic_link_path(today_dir_path: &PathBuf) -> Result<PathBuf, String> {
    log::debug!("get_today_symbolic_link_path start");

    let parent_dir_path = match today_dir_path.parent() {
        Some(path) => path.to_path_buf(),
        None => return Err("Couldn't get the parent directory path.".to_string()),
    };
    let mut today_link_path = parent_dir_path.clone();
    today_link_path.push("today");
    return Ok(today_link_path);
}

/// Update "today" symbolic link
///
/// If an old link exists, it removes that beforehand.
///
#[cfg(target_os = "linux")]
fn update_today_symbolic_link(today_dir_path: &PathBuf) -> Result<PathBuf, String> {
    log::debug!("update_today_symbolic_link (linux) start");

    let link_path = get_today_symbolic_link_path(today_dir_path)?;
    if link_path.exists() {
        match remove_file(&link_path) {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }
    }
    // Wait 1 second before link creation.
    thread::sleep(Duration::from_secs(1));
    return create_today_symbolic_link(today_dir_path);
}

/// Create "today" symbolic link
#[cfg(target_os = "linux")]
fn create_today_symbolic_link(today_dir_path: &PathBuf) -> Result<PathBuf, String> {
    log::debug!("create_today_symbolic_link (linux) start");
    use std::os::unix::fs::symlink;

    let today_link_path = get_today_symbolic_link_path(today_dir_path)?;
    return match symlink(&today_dir_path, &today_link_path) {
        Ok(_) => Ok(today_link_path),
        Err(e) => Err(e.to_string()),
    };
}
/// Create "today" symbolic link
#[cfg(target_os = "windows")]
fn create_today_symbolic_link(today_dir_path: &PathBuf) -> Result<PathBuf, String> {
    use std::os::windows::fs::symlink_dir;

    // Get the parent dir path
    let parent_dir_path = match today_dir_path.parent() {
        Some(path) => path.to_path_buf(),
        None => return Err("Couldn't get the parent directory path.".to_string()),
    };

    // Create "today" symbolic link
    let mut today_link_path = parent_dir_path.clone();
    today_link_path.push("today");
    return match symlink_dir(today_dir_path, today_link_path) {
        Ok(_) => today_link_path,
        Err(e) => e.to_string(),
    };
}
// -----------------------------------------------------------------------------
/// Tests class
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    // Cargo libraries ------------------
    use chrono::NaiveDate;

    // Functions ------------------------
    use super::get_dir_name_of;

    #[test]
    fn test_get_dir_name_of_001_1900_01_02() {
        let date = NaiveDate::from_ymd_opt(1900, 1, 2).unwrap();
        let dir_name = get_dir_name_of(date);
        assert_eq!(dir_name, "1900-01-02");
    }
}
