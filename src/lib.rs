//! golden-dawn : Daily Directory Maker

// Standard libraries ------------------
// Cargo libraries ----------------------
use chrono::{Datelike, Local, NaiveDate};
// Modules ------------------------------
mod util;
// ---------------------------------------

/// The main function of golden-dawn.
pub fn main() {
    let today = Local::now().date_naive();
    println!("{}", get_dir_name_of(today))
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
