use clap::ValueEnum;

use crate::{util, commands::auth, spotify::{self, types::Token}};

#[derive(ValueEnum, Clone, Copy)]
pub enum CopyPreposition {
    From,
    Into,
    IntoNew,
}

pub fn run(path: std::path::PathBuf, a: &str, prep: &CopyPreposition, b: &str) {
    // Token acquisition
    let config = util::read_config(&path);
    let token = auth::refresh(&config);
    if token.is_err() {
        println!("Failed to auth: {:?}", token.unwrap_err());
        return;
    }
    let token = token.unwrap();

    match prep {
        CopyPreposition::From => {
            println!("Copying into {} from {}", a, b);
            run_order_const(&token, b, a)
        }
        CopyPreposition::Into => {
            println!("Copying from {} into {}", a, b);
            run_order_const(&token, a, b)
        }
        CopyPreposition::IntoNew => {
            println!("Copying from {} into new playlist: {}", a, b);
            run_into_new(&token, a, b)
        }
    }
}

fn run_order_const(token: &Token, from: &str, to: &str) {
    let new_uris = spotify::playlist::get_playlist_uris(from, token);
    match new_uris {
        Ok(uris) => {
            let res = spotify::playlist::add_to_playlist(to, &uris, token);
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

fn run_into_new(token: &Token, from: &str, to: &str) {
    let new_playlist = spotify::playlist::create_playlist(to, token);

    match new_playlist {
        Ok((uri, link)) => {
            let new_uris = spotify::playlist::get_playlist_uris(from, token);
            match new_uris {
                Ok(uris) => {
                    let res = spotify::playlist::set_playlist(&uri.0, &uris, token);
                    match res {
                        Ok(_) => {
                            println!("Copy complete.\nURI:{}\nLink:{}", uri, link);
                        }
                        Err(e) => {
                            println!("Failed to set playlist contents:\n{:?}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to get from playlist\n{:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to make a new playlist:\n{:?}", e);
        }
    }

}
