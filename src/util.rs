//! One file util module

// Standard libraries ------------------
use std::env::current_exe;
use std::fs::create_dir_all;
use std::io::Result;
use std::path::PathBuf;
// Cargo libraries ----------------------
// Modules ------------------------------
// ---------------------------------------

/// It returns dir path of executable file.  
/// If it is failure, it returns clone of fallback_path.
#[allow(dead_code)]
pub fn get_dir_path_of_exe(fallback_path: &PathBuf) -> PathBuf {
    return match current_exe() {
        Ok(exe_path) => match exe_path.parent() {
            Some(dir_path) => dir_path.to_path_buf(),
            None => fallback_path.clone(),
        },
        Err(_) => fallback_path.clone(),
    };
}

/// It try to create a target directory and its upper directories.
/// If the target directory is exist already, it returns Ok.
#[allow(dead_code)]
pub fn create_dir(target_dir_path: &PathBuf) -> Result<()> {
    return create_dir_all(target_dir_path);
}
