//! golden-dawn : Daily Directory Maker

// Standard libraries ------------------
use std::fs;
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

    // Update "today" link
    match update_today_link(&today_dir_path) {
        Ok(_) => (),
        Err(e) => {
            log::error!("{}", e);
            return;
        }
    }

    // Move old directories
    match move_old_dirs(&parent_dir_path, 7) {
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
/// It return parent dir path of daily directories and "today" link.
///
/// # TODO
///
/// * It should get the path from a setting file.
///
fn get_parent_dir_path() -> Result<PathBuf, String> {
    let current_dir = Path::new(".").to_path_buf();
    return Ok(current_dir);
}

/// Find old directoris
///
/// # Returns:
/// 
/// It returns paths of old direcotories exceed threshold.
/// 
/// 
fn find_old_dirs(dir_about: &PathBuf, threshold_days: u32) -> Result<Vec<PathBuf>, String> {
    log::debug!("find_old_dirs start");

    // List entries of dir_about
    let read_dir_result = match fs::read_dir(dir_about) {
        Ok(result) => result,
        Err(e) => {
            log::error!("read_dir failed.");
            return Err(e.to_string())
        }
    };

    // Vec to return
    let mut old_dir_list = Vec::<PathBuf>::new();

    // Push directories enough old.
    for result_item in read_dir_result {
        let dir_entry = match result_item {
            Ok(item) => item,
            Err(e) => {
                log::error!("Failed to get a DirEntry.");
                return Err(e.to_string())
            }
        };

        if !dir_entry.path().is_dir() {
            continue;
        }

        let dir_name_os_str = dir_entry.file_name();
        let dir_name = match dir_name_os_str.to_str() {
            Some(name) => name,
            None => {
                log::warn!("Failed to get dir_name.");
                continue
            }
        };

        let date_of_dir = match NaiveDate::parse_from_str(dir_name, "%Y-%m-%d") {
            Ok(data) => data,
            Err(_) => {
                log::warn!("NaiveDate::parse_from_str failed. ({})", dir_name);
                continue
            }
        };

        let today = Local::now().date_naive();
        let naive_date_diff = today - date_of_dir;
        if naive_date_diff.num_days() > threshold_days as i64 {
            old_dir_list.push(dir_entry.path().clone())
        }
    }

    return Ok(old_dir_list);
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
                "Couldn't create the directory of today. error:{}",
                e.to_string()
            );
            Err(error_message)
        }
    };
}

//. Get "today" link path
fn get_today_link_path(today_dir_path: &PathBuf) -> Result<PathBuf, String> {
    log::debug!("get_today_link_path start");

    let parent_dir_path = match today_dir_path.parent() {
        Some(path) => path.to_path_buf(),
        None => return Err("Couldn't get the parent directory path.".to_string()),
    };
    let mut today_link_path = parent_dir_path.clone();
    today_link_path.push("today");
    return Ok(today_link_path);
}

/// Update "today" link
///
/// If an old link exists, it removes that beforehand.
/// Linux version create "today" symbolic link.
///
#[cfg(target_os = "linux")]
fn update_today_link(today_dir_path: &PathBuf) -> Result<PathBuf, String> {
    log::debug!("update_today_link (linux) start");
    use std::fs::remove_file;

    let link_path = get_today_link_path(today_dir_path)?;
    if link_path.exists() {
        match remove_file(&link_path) {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }
    }
    // Wait 1 second before link creation.
    thread::sleep(Duration::from_secs(1));
    return create_today_link(today_dir_path);
}

/// Create "today" link
/// 
/// Linux version create "today" symbolic link.
/// 
#[cfg(target_os = "linux")]
fn create_today_link(today_dir_path: &PathBuf) -> Result<PathBuf, String> {
    log::debug!("create_today_link (linux) start");
    use std::os::unix::fs::symlink;

    let today_link_path = get_today_link_path(today_dir_path)?;
    return match symlink(&today_dir_path, &today_link_path) {
        Ok(_) => Ok(today_link_path),
        Err(e) => Err(e.to_string()),
    };
}

/// Update "today" link
///
/// If an old link exists, it removes that beforehand.
/// Windows version create "today" junction.
///
#[cfg(target_os = "windows")]
fn update_today_link(today_dir_path: &PathBuf) -> Result<PathBuf, String> {
    use std::fs::remove_dir;

    log::debug!("update_today_link (windows) start");

    let link_path = get_today_link_path(today_dir_path)?;
    if link_path.exists() {
        // Change "today" junction to a just direcotry beforehand.
        match junction::delete(&link_path) {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }
        // Remove
        match remove_dir(&link_path) {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }
    }
    // Wait 1 second before link creation.
    thread::sleep(Duration::from_secs(1));
    return create_today_link(today_dir_path);
}

/// Create "today" link
#[cfg(target_os = "windows")]
fn create_today_link(today_dir_path: &PathBuf) -> Result<PathBuf, String> {
    log::debug!("create_today_link (windows) start");

    let today_link_path = get_today_link_path(today_dir_path)?;
    return match junction::create(&today_dir_path, &today_link_path) {
        Ok(_) => Ok(today_link_path),
        Err(e) => Err(e.to_string()),
    };
}

/// Move old directories in "old" directory.
fn move_old_dirs(parent_dir_path: &PathBuf, threshold_days: u32) -> Result<(), String>{
    let old_dir = create_old_dir(parent_dir_path)?;
    let old_dirs = find_old_dirs(parent_dir_path, threshold_days)?;
    for old_dir_from in old_dirs.into_iter() {
        let mut old_dir_to = old_dir.clone();
        let old_dir_src_name = match old_dir_from.file_name() {
            Some(name) => name,
            None => return Err("old_dir_from.file_name() failed".to_string())
        };
        old_dir_to.push(old_dir_src_name);
        match fs::rename(old_dir_from, old_dir_to) {
            Ok(_) => (),
            Err(e) => {
                let log_message = format!(
                    "Moving an old direcoty failed. error:{}",
                    e.to_string());
                return Err(log_message);
            }
        };
    }
    return Ok(());
}

/// Create "old" directory
fn create_old_dir(parent_dir_path: &PathBuf) -> Result<PathBuf, String> {

    let mut old_dir_path = parent_dir_path.clone();
    old_dir_path.push("old");

    // Create target directory
    return match util::create_dir(&old_dir_path) {
        Ok(_) => Ok(old_dir_path),
        Err(e) => {
            let error_message = format!(
                "Couldn't create the \"old\" directory. error:{}",
                e.to_string()
            );
            Err(error_message)
        }
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
