use clap::ValueEnum;

use crate::{util, commands::auth, spotify};

#[derive(ValueEnum, Clone, Copy)]
pub enum CopyPreposition {
    From,
    Into,
}

pub fn run(path: std::path::PathBuf, a: &str, prep: &CopyPreposition, b: &str) {
    match prep {
        CopyPreposition::From => {
            println!("Copying into {} from {}", a, b);
            run_order_const(path, b, a)
        }
        CopyPreposition::Into => {
            println!("Copying from {} into {}", a, b);
            run_order_const(path, a, b)
        }
    }
}

fn run_order_const(path: std::path::PathBuf, from: &str, to: &str) {
    // Token acquisition
    let config = util::read_config(&path);
    let token = auth::refresh(&config);
    if token.is_err() {
        println!("Failed to auth: {:?}", token.unwrap_err());
        return;
    }
    let token = token.unwrap();

    let new_uris = spotify::playlist::get_playlist_uris(from, &token);
    match new_uris {
        Ok(uris) => {
            let res = spotify::playlist::add_to_playlist(to, &uris, &token);
            match res {
                Ok(_) => {
                    println!("Copy complete.")
                },
                Err(e) => {
                    println!("Failed to add to playlist\n{:?}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to get from playlist\n{:?}", e);
        }
    }
}
