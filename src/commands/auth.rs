use serde::Deserialize;

use std::io::{prelude::*, BufReader, Write};
use std::net::TcpListener;
use std::path::PathBuf;

use crate::spotify::types::Token;
use crate::util::{self, Config};

#[derive(Deserialize, Debug)]
#[non_exhaustive]
struct AuthorizationRequestResponse {
    pub _access_token: String,
    pub refresh_token: String,
}

pub fn run(path: PathBuf) {
    const REDIRECT_URI: &str = "http://localhost:8888";
    // Try to read our config, die if it isn't there!
    let config = util::read_config(&path);
    if config.client_id.is_none() || config.client_secret.is_none() {
        panic!("Cannot authorize! No client id or secret.");
    }

    if config.refresh_token.is_some() {
        println!("Config already contains a token.\nDelete the \"refresh_token\" line and run this command again to reauthorize.");
        return;
    }

    let client_id = config.client_id.unwrap();
    let client_secret = config.client_secret.unwrap();

    // Build a URL, I just ripped this from cool-spotify-blend
    let auth_url = format!("https://accounts.spotify.com/authorize?client_id={}&scope={}&redirect_uri={}&response_type={}",
        client_id,
        "user-top-read playlist-modify-public playlist-modify-private",
        REDIRECT_URI,
        "code");

    // User has to click this link and authorize
    println!("Open the following link and authorize:\n{}", auth_url);

    let listener = TcpListener::bind("127.0.0.1:8888").expect("Failed to bind to port 8888!");
    let data: String = match listener.accept() {
        Ok((mut stream, _addr)) => {
            println!("Connection opened.");
            write!(&mut stream, "HTTP/1.1 204 No Content\r\n\r\n")
                .expect("Failed to send response");

            let buf_reader = BufReader::new(&mut stream);
            buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take(1)
                .collect()
        }
        Err(..) => panic!("Failed to get client"),
    };
    // This extracts the code from our data,
    // skips characters until code starts
    // and takes until a space indicating the code is over
    let code: String = data
        .chars()
        .skip_while(|c| *c != '?')
        .skip(6)
        .take_while(|c| *c != ' ')
        .collect();

    println!("{}", code);

    // Get the token and more importantly refresh token from spotify
    let params = vec![
        ("grant_type", "authorization_code"),
        ("code", &code),
        ("redirect_uri", REDIRECT_URI),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
    ];
    let res: AuthorizationRequestResponse = ureq::post("https://accounts.spotify.com/api/token")
        .set("Content-Type", "application/x-www-form-urlencoded")
        .set("Content-Length", "0")
        .query_pairs(params)
        .call()
        .expect("Failed to get token")
        .into_json()
        .expect("Failed to parse token response");

    util::write_config(
        path,
        util::Config {
            client_id: Some(client_id),
            client_secret: Some(client_secret),
            refresh_token: Some(res.refresh_token),
        },
    );

    println!("Authorization completed.");
}

#[derive(Deserialize)]
#[non_exhaustive]
struct RefreshRes {
    access_token: String,
}

#[derive(Debug)]
pub enum RefreshError {
    Request(Box<ureq::Error>),
    Json(std::io::Error),
    Config {
        missing_refresh_token: bool,
        missing_client_id: bool,
        missing_client_secret: bool,
    },
}

impl Config {
    pub fn check_config(&self) -> (bool, bool, bool) {
        let mut missing = (false, false, false);
        if self.refresh_token.is_none() {
            missing.0 = true
        }
        if self.client_id.is_none() {
            missing.1 = true
        }
        if self.client_secret.is_none() {
            missing.2 = true
        }

        missing
    }

    pub fn is_valid(&self) -> bool {
        let triple = self.check_config();
        !(triple.0 || triple.1 || triple.2)
    }
}

pub fn refresh(config: &util::Config) -> Result<Token, RefreshError> {
    if !config.is_valid() {
        let missing = config.check_config();
        return Err(RefreshError::Config {
            missing_refresh_token: missing.0,
            missing_client_id: missing.1,
            missing_client_secret: missing.2,
        });
    }
    let refresh_token = config.refresh_token.clone().unwrap();
    let client_id = config.client_id.clone().unwrap();
    let client_secret = config.client_secret.clone().unwrap();

    let res = ureq::post("https://accounts.spotify.com/api/token")
        .set("Content-Type", "application/x-www-form-urlencoded")
        .set("Content-Length", "0")
        .query_pairs(vec![
            ("grant_type", "refresh_token"),
            ("refresh_token", &refresh_token),
            ("client_id", &client_id),
            ("client_secret", &client_secret),
        ])
        .call();

    if let Err(e) = res {
        return Err(RefreshError::Request(Box::new(e)));
    }

    match res.unwrap().into_json::<RefreshRes>() {
        Err(e) => Err(RefreshError::Json(e)),
        Ok(t) => Ok(Token(t.access_token)),
    }
}
