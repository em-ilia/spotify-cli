use crate::spotify::user::TopTerm;
use crate::spotify::types::Uri;

#[cfg(feature = "json_export")]
use serde_json;

pub fn run(
    path: std::path::PathBuf,
    term: &super::Term,
    number: u16,
    name: Option<&str>,
    dump: Option<bool>,
) {
    let token = crate::util::get_token(&path);

    let tracks = crate::spotify::user::get_user_top_tracks(&token, &(Into::<TopTerm>::into(*term)), number);
    if let Err(e) = tracks {
        println!("Failed to get top tracks:\n{:?}", e);
        return;
    }
    let tracks = tracks.unwrap();

    if dump.is_some_and(|b| b) {
        if cfg!(feature = "json_export") {
            #[cfg(feature = "json_export")]
            {
                match serde_json::to_string(&tracks) {
                    Ok(data) => {
                        println!("{}", data);
                    }
                    Err(e) => {
                        println!("Failed to serialize tracks:\n{:?}", e);
                    }
                }
            }
        } else {
            println!("{:?}", tracks);
        }
        return;
    }
    // Past here we know that name must have had a value, otherwise clap would terminate
    let name = name.expect("Clap should have ensured this was required");
    let track_uris: Vec<Uri> = tracks.into_iter().map(|t| t.uri).collect();

    let new_playlist_res = crate::spotify::playlist::create_playlist(name, &token);
    if let Err(e) = new_playlist_res {
        println!("Failed to create new playlist:\n{:?}", e);
        return;
    }
    let new_playlist_res = new_playlist_res.unwrap();
    
    let set_res = crate::spotify::playlist::set_playlist(&new_playlist_res.0.0, &track_uris, &token);
    match set_res {
        Ok(_) => {
            println!("Playlist creation complete.\nURI: {}\nLink: {}", new_playlist_res.0, new_playlist_res.1);
        }
        Err(e) => {
            println!("Failed to set playlist contents:\n{:?}", e);
            return;
        }
    }
}
