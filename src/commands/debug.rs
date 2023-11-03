use std::path::PathBuf;

#[cfg(feature = "json_export")]
use serde_json;

use crate::{commands::auth, spotify, util};

pub fn dump_playlist(path: PathBuf, playlist: &str) {
    let token = util::get_token(&path);

    let res = spotify::playlist::get_playlist_items(playlist, &token);

    // We will use serde_json to export as JSON if the "json_export" feature is enabled,
    // otherwise, we'll just dump the derived Debug of everything.
    if cfg!(feature = "json_export") {
        #[cfg(feature = "json_export")]
        {
        match res {
            Ok(data) => {
                let table = serde_json::to_string(&data);
                match table {
                    Ok(table) => {
                        println!("{}", table.to_string());
                    }
                    Err(e) => {
                        println!("Failed to serialize data:\n{:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("Failed to get playlist items:\n{:?}", e);
            }
        }
        }
    } else {
        println!("{:?}", res);
    }
}

pub fn clear_playlist(path: PathBuf, playlist: &str) {
    let token = util::get_token(&path);

    let res = spotify::playlist::set_playlist(playlist, &[], &token);

    match res {
        Ok(_) => println!("Playlist cleared."),
        Err(e) => println!("Failed to clear playlist:\n{:?}", e)
    }
}
