use std::collections::HashMap;

use serde::Deserialize;

use crate::spotify::types::{PlaylistTrackObject, Token, TrackObject, Uri};

pub fn add_to_playlist(id: &str, uris: &[Uri], token: &Token) -> Result<(), reqwest::Error> {
    for chunk in uris.chunks(100) {
        add_to_playlist_helper(id, chunk, token)?;
    }

    Ok(())
}

fn add_to_playlist_helper(id: &str, uris: &[Uri], token: &Token) -> Result<(), reqwest::Error> {
    let mut params = HashMap::new();
    params.insert("position", "0"); // Insert at top, we could remove to append

    let mut body: HashMap<String, Vec<String>> = HashMap::new();
    let uris: Vec<String> = uris.iter().map(|u| &u.0).cloned().collect();
    body.insert("uris".to_owned(), uris);

    let client = reqwest::blocking::Client::new();
    let _res = client
        .post(crate::spotify::BASE_URL.to_owned() + "/playlists/" + id + "/tracks")
        .query(&params)
        .json(&body)
        .header("Authorization", "Bearer ".to_owned() + &token.0)
        .send()?;

    Ok(())
}

pub fn set_playlist(id: &str, uris: &[Uri], token: &Token) -> Result<(), reqwest::Error> {
    if uris.len() == 0 {
        set_playlist_helper(id, &Vec::new(), token)?;
        return Ok(());
    }

    for chunk in uris.chunks(100) {
        set_playlist_helper(id, chunk, token)?;
    }

    Ok(())
}

// No more than 100 to be passed into this function!
fn set_playlist_helper(id: &str, uris: &[Uri], token: &Token) -> Result<(), reqwest::Error> {
    let mut body: HashMap<String, Vec<String>> = HashMap::new();
    let uris: Vec<String> = uris.iter().map(|u| &u.0).cloned().collect();
    body.insert("uris".to_owned(), uris);

    let client = reqwest::blocking::Client::new();
    let _res = client
        .put(crate::spotify::BASE_URL.to_owned() + "/playlists/" + id + "/tracks")
        .json(&body)
        .header("Authorization", "Bearer ".to_owned() + &token.0)
        .send()?;

    Ok(())
}

#[derive(Deserialize)]
struct GetPlaylistItemsRes {
    pub items: Vec<PlaylistTrackObject>,
    pub next: Option<String>,
}

pub fn get_playlist_items(id: &str, token: &Token) -> Result<Vec<TrackObject>, reqwest::Error> {
    let mut tracks: Vec<TrackObject> = Vec::new();
    let mut done = false;
    let mut offset = 0u16;

    let mut client = reqwest::blocking::Client::new();

    while !done {
        match get_playlist_items_helper(id, token, &mut client, offset) {
            Ok((playlist_tracks, is_done)) => {
                match is_done {
                    false => offset += 50,
                    true => done = true,
                }

                tracks.extend(playlist_tracks.into_iter().map(|p| p.track));
            }
            Err(e) => return Err(e),
        }
    }

    Ok(tracks)
}

pub fn get_playlist_uris(id: &str, token: &Token) -> Result<Vec<Uri>, reqwest::Error> {
    Ok(get_playlist_items(id, token)?.into_iter().map(|t| t.uri).collect())
}

fn get_playlist_items_helper(
    id: &str,
    token: &Token,
    client: &mut reqwest::blocking::Client,
    offset: u16,
) -> Result<(Vec<PlaylistTrackObject>, bool), reqwest::Error> {
    // Returns Ok(vec![list of uris], true) if there is no next page
    // Returns Ok(vec![list of uris], false) if we need to keep fetching

    let res: GetPlaylistItemsRes = client
        .get(crate::spotify::BASE_URL.to_owned() + "/playlists/" + id + "/tracks")
        .header("Authorization", "Bearer ".to_owned() + &token.0)
        .query(&[("offset", offset), ("limit", 50)])
        .send()?
        .json()?;

    match res.next.as_deref() {
        None => Ok((res.items, true)),
        Some("null") => Ok((res.items, true)),
        Some(..) => Ok((res.items, false)),
    }
}
