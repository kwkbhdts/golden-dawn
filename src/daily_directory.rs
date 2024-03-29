use std::fs;
use std::fs::DirEntry;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use chrono::{Local, NaiveDate, Days};
use fs_extra::dir;
use super::error_util;

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
            date: Local::now().naive_local().date(),
        }
    }

    pub fn search_daily_directories(
        dir_path: &'a Path,
        date_format: &'a String
    ) -> io::Result<Vec<DailyDirectory<'a>>> {
        let mut daily_directory_vec = Vec::new();
        let read_dir = fs::read_dir(dir_path)?;
        for dir_entry_result in read_dir {
            let dir_entry = dir_entry_result?;
            if let Some(daily_directory) =
                Self::from_dir_entry(dir_path, date_format, dir_entry) {
                daily_directory_vec.push(daily_directory);
            }
        }
        Ok(daily_directory_vec)
    }

    /// Returns whether this DailyDirectory is old.
    pub fn is_old(&self, days_to_old: u32) -> bool {
        if days_to_old == 0 {
            return false;
        }
        let today = Local::now().naive_local().date();
        let elapsed_days = (today - self.date).num_days();
        elapsed_days >= (days_to_old as i64)
    }

    /// Create a directory of this DailyDirectory.
    pub fn create(
        &self,
        template_dir_path: &Path,
        previous_dir_junction_name: &String
    ) -> error_util::Result<()> {
        let dir_path = self.create_dir_path();
        if dir_path.exists() {
            return Ok(());
        }
        fs::create_dir_all(&dir_path)?;

        // Copyt template files.
        let mut copy_options = dir::CopyOptions::new();
        copy_options.copy_inside = true;
        copy_options.content_only = true;
        dir::copy(template_dir_path, &dir_path, &copy_options)?;

        // Create a junction to a previous date directory.
        if let Some(previous_dir_path) = self.create_previous_date_dir_path() {
            if previous_dir_path.exists() {
                let junction_path = dir_path.join(previous_dir_junction_name);
                junction::create(previous_dir_path, junction_path)?;
            }
        }

        Ok(())
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

    /// Move directory of this DailyDirectory.
    pub fn move_to(&self, new_parent_dir_path: &'a Path) -> io::Result<()> {
        let src = self.create_dir_path();
        let dir_name = self.date.format(self.date_format).to_string();
        let dst = new_parent_dir_path.join(dir_name);
        fs::rename(src, dst)
    }

    /// Remove directory of this DailyDirectory.
    pub fn remove(&self) -> io::Result<()> {
        fs::remove_dir_all(self.create_dir_path())
    }

    /// Create a dir path of this DailyDirectory.
    pub fn create_dir_path(&self) -> PathBuf {
        let dir_name = self.date.format(self.date_format).to_string();
        self.parent_dir_path.join(dir_name)
    }

    /// Create a dir path of this DailyDirectory.
    pub fn create_previous_date_dir_path(&self) -> Option<PathBuf> {
        let previous_date = self.date.checked_sub_days(Days::new(1))?;
        let dir_name = previous_date.format(self.date_format).to_string();
        Some(self.parent_dir_path.join(dir_name))
    }

    // Create a DailyDirectory from a DirEntry.
    fn from_dir_entry(
        dir_path: &'a Path,
        date_format: &'a String,
        dir_entry: DirEntry
    ) -> Option<DailyDirectory<'a>> {
        // Return None if dir_entry is not a directory.
        if let Ok(metadata) = dir_entry.metadata() {
            if !metadata.is_dir() {
                return None;
            }
        } else {
            return None;
        }

        // Create and return a DailyDirectory.
        if let Ok(daily_dir_name) = dir_entry.file_name().into_string() {
            if let Ok(date) = NaiveDate::parse_from_str(&daily_dir_name, date_format) {
                return Some(DailyDirectory {
                    parent_dir_path: dir_path,
                    date_format: date_format,
                    date: date,
                });
            }
        }
        None
    }
}
