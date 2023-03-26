mod config;

use config::Config;

const CONFIG_FILE_PATH: &str = "resource/config.toml";

fn main() {
    let config = Config::from_file(&CONFIG_FILE_PATH.to_string())
        .expect("Failed to load a config file.");
    println!("{:?}", config);
}
