use std::path::PathBuf;

use crate::{auth, spotify, util};

pub fn dump_playlist(path: PathBuf, playlist: &str) {
    let config = util::read_config(&path);
    let token = auth::refresh(&config);
    if token.is_err() {
        println!("Failed to get auth: {:?}", token.unwrap_err());
        return;
    }
    let token = token.unwrap();

    let res = spotify::playlist::get_playlist_items(playlist, &token);

    println!("{:?}", res);
}

pub fn clear_playlist(path: PathBuf, playlist: &str) {
    let config = util::read_config(&path);
    let token = auth::refresh(&config);
    if token.is_err() {
        println!("Failed to get auth: {:?}", token.unwrap_err());
        return;
    }
    let token = token.unwrap();

    let res = spotify::playlist::set_playlist(playlist, &[], &token);

    match res {
        Ok(_) => println!("Playlist cleared."),
        Err(e) => println!("Failed to clear playlist:\n{:?}", e)
    }
}
