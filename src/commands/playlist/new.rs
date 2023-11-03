use crate::{util, commands::auth, spotify};

pub fn run(path: std::path::PathBuf, name: &str) {
    let token = util::get_token(&path);

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
