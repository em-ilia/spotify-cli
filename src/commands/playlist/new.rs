use crate::{util, commands::auth, spotify};

pub fn run(path: std::path::PathBuf, name: &str) {
    // Token acquisition
    let config = util::read_config(&path);
    let token = auth::refresh(&config);
    if token.is_err() {
        println!("Failed to auth: {:?}", token.unwrap_err());
        return;
    }
    let token = token.unwrap();

    let res = spotify::playlist::create_playlist(name, &token);

    match res {
        Ok((uri, href)) => {
            println!("Creation completed.\nURI: {}\nLink: {}", uri, href);
        }
        Err(e) => {
            println!("Playlist creation failed:\n{:?}", e);
        }
    }
}
