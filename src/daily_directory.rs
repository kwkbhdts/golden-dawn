use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use chrono::{Local, NaiveDate};

/// Daily directory
#[derive(Debug)]
pub struct DailyDirectory<'a> {
    parent_dir_path: &'a Path,
    date_format: &'a String,
    date: NaiveDate,
}

impl<'a> DailyDirectory<'a> {
    /// Return DailyDirectory of today.
    pub fn direcory_of_today(
        parent_dir_path: &'a Path,
        date_format: &'a String
    ) -> DailyDirectory<'a> {
        DailyDirectory {
            parent_dir_path: parent_dir_path,
            date_format: date_format,
            date: Local::now().naive_utc().date(),
        }
    }

    /// Create a directory of this DailyDirectory.
    pub fn create(&self) -> io::Result<()> {
        fs::create_dir_all(self.create_dir_path())
    }

    /// Create a junction of this DailyDirectory.
    pub fn create_junction(
        &self,
        junction_path_from_parent: &Path
    ) -> io::Result<()> {
        let junction_path = self.parent_dir_path.join(junction_path_from_parent);

        // Remove the old one if exists.
        if junction_path.exists() {
            fs::remove_dir(&junction_path)?
        }

        junction::create(self.create_dir_path(), junction_path)
    }

    /// Create a dir path of this DailyDirectory.
    fn create_dir_path(&self) -> PathBuf {
        let dir_name = self.date.format(self.date_format).to_string();
        self.parent_dir_path.join(dir_name)
    }
}
