use std::env;
use std::env::VarError;
use std::fs;
use std::path::{Path, PathBuf};

const ENV_CARGO_DIR: &str = "CARGO_MANIFEST_DIR";
const ENV_PROFILE: &str = "PROFILE";
const CARGO_TARGET_DIR: &str = "target";
const COPY_INFO: [(&str, &str); 1] =
    [("resource/config.toml", "config.toml")];

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current dir.");
    let target_dir = get_target_dir().expect("Failed to get target dir.");

    for x in COPY_INFO {
        let src = current_dir.join(Path::new(x.0));
        let dst = target_dir.join(Path::new(x.1));
        let _ = fs::copy(src, dst);
    }
}

fn get_target_dir() -> Result<PathBuf, VarError> {
    let cargo_dir = env::var(ENV_CARGO_DIR)?;
    let build_type = env::var(ENV_PROFILE)?;
    let path = Path::new(&cargo_dir).join(CARGO_TARGET_DIR).join(build_type);
    Ok(PathBuf::from(path))
}
