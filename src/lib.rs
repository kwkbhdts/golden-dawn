//! golden-dawn : Daily Directory Maker

// Standard libraries ------------------
use std::path::Path;
// Cargo libraries ----------------------
use chrono::{Datelike, Local, NaiveDate};
// Modules ------------------------------
mod util;
// ---------------------------------------

/// The main function of golden-dawn.
pub fn main() {
    // Init logger
    env_logger::init();

    // Get target directory path
    let today = Local::now().date_naive();
    let current_dir = Path::new(".").to_path_buf();
    let mut target_dir_path = util::get_dir_path_of_exe(&current_dir);
    target_dir_path.push(get_dir_name_of(today));
    log::info!("target_path = {}", target_dir_path.to_str().unwrap());

    // Create target directory
    match util::create_dir(&target_dir_path) {
        Ok(_) => (),
        Err(e) => log::error!(
            "Couldn't create the directory. ({}) : {}",
            target_dir_path.to_str().unwrap(),
            e.to_string())
    }
}

/// It converts a NaiveDate to a direcory name string.
fn get_dir_name_of(date: NaiveDate) -> String {
    return format!("{}-{:02}-{:02}", date.year(), date.month(), date.day());
}

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
