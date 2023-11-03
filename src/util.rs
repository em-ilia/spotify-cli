use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UreqOrJSONError {
    #[error("Request failed or got 4xx code")]
    Request(#[from] ureq::Error),
    #[error("Failure to parse JSON")]
    Json(#[from] std::io::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub refresh_token: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

pub fn read_config(path: &PathBuf) -> Config {
    let path = PathBuf::from(
        shellexpand::tilde(&path.to_path_buf().into_os_string().into_string().ok().unwrap()).to_string(),
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

pub fn get_token(path: &PathBuf) -> crate::spotify::types::Token {
    // Token acquisition
    let config = crate::util::read_config(&path);
    let token = crate::commands::auth::refresh(&config);
    if token.is_err() {
        println!("Failed to auth: {:?}", token.unwrap_err());
        std::process::exit(1);
    }
    token.unwrap()
}
