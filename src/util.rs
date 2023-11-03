use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use shellexpand;
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub refresh_token: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

pub fn read_config(path: &PathBuf) -> Config {
    let path = PathBuf::from(
        shellexpand::tilde(&path.clone().into_os_string().into_string().ok().unwrap()).to_string(),
    ).join("config");

    if !path.is_file() {
        Config {
            refresh_token: None,
            client_id: None,
            client_secret: None,
        }
    } else {
        let data = fs::read_to_string(path).expect("Failed to read file!");
        toml::from_str(&data).expect("Failed to parse toml")
    }
}

pub fn write_config(path: PathBuf, config: Config) {
    let path = PathBuf::from(
        shellexpand::tilde(&path.clone().into_os_string().into_string().ok().unwrap()).to_string(),
    ).join("config");

    let data: String = toml::to_string(&config).unwrap();

    fs::write(path, data).ok();
}
