use std::collections::HashMap;

use serde::Deserialize;

use crate::spotify::types::{PlaylistTrackObject, Token, TrackObject, Uri};
use crate::util::UreqOrJSONError;

pub fn add_to_playlist(id: &str, uris: &[Uri], token: &Token) -> Result<(), ureq::Error> {
    for chunk in uris.chunks(100) {
        add_to_playlist_helper(id, chunk, token)?;
    }

    Ok(())
}

fn add_to_playlist_helper(id: &str, uris: &[Uri], token: &Token) -> Result<(), ureq::Error> {
    let mut body: HashMap<String, Vec<String>> = HashMap::new();
    let uris: Vec<String> = uris.iter().map(|u| &u.0).cloned().collect();
    body.insert("uris".to_owned(), uris);

    let _res = ureq::post(&(crate::spotify::BASE_URL.to_owned() + "/playlists/" + id + "/tracks"))
        .query("position", "0")
        .set("Authorization", &("Bearer ".to_owned() + &token.0))
        .send_json(body)?;

    Ok(())
}

pub fn set_playlist(id: &str, uris: &[Uri], token: &Token) -> Result<(), ureq::Error> {
    if uris.is_empty() {
        set_playlist_helper(id, &Vec::new(), token)?;
        return Ok(());
    }

    for chunk in uris.chunks(100) {
        set_playlist_helper(id, chunk, token)?;
    }

    Ok(())
}

// No more than 100 to be passed into this function!
fn set_playlist_helper(id: &str, uris: &[Uri], token: &Token) -> Result<(), ureq::Error> {
    let mut body: HashMap<String, Vec<String>> = HashMap::new();
    let uris: Vec<String> = uris.iter().map(|u| &u.0).cloned().collect();
    body.insert("uris".to_owned(), uris);

    let _res = ureq::put(&(crate::spotify::BASE_URL.to_owned() + "/playlists/" + id + "/tracks"))
        .set("Authorization", &("Bearer ".to_owned() + &token.0))
        .send_json(&body)?;

    Ok(())
}

#[derive(Deserialize)]
struct GetPlaylistItemsRes {
    pub items: Vec<PlaylistTrackObject>,
    pub next: Option<String>,
}

pub fn get_playlist_items(id: &str, token: &Token) -> Result<Vec<TrackObject>, UreqOrJSONError> {
    let mut tracks: Vec<TrackObject> = Vec::new();
    let mut done = false;
    let mut offset = 0u16;

    let agent = ureq::Agent::new();

    while !done {
        match get_playlist_items_helper(id, token, &agent, offset) {
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

pub fn get_playlist_uris(id: &str, token: &Token) -> Result<Vec<Uri>, UreqOrJSONError> {
    Ok(get_playlist_items(id, token)?
        .into_iter()
        .map(|t| t.uri)
        .collect())
}

fn get_playlist_items_helper(
    id: &str,
    token: &Token,
    agent: &ureq::Agent,
    offset: u16,
) -> Result<(Vec<PlaylistTrackObject>, bool), UreqOrJSONError> {
    // Returns Ok(vec![list of uris], true) if there is no next page
    // Returns Ok(vec![list of uris], false) if we need to keep fetching

    let res: GetPlaylistItemsRes = agent
        .get(&(crate::spotify::BASE_URL.to_owned() + "/playlists/" + id + "/tracks"))
        .set("Authorization", &("Bearer ".to_owned() + &token.0))
        .query_pairs(vec![
            ("offset", offset.to_string().as_str()),
            ("limit", "50"),
        ])
        .call()?
        .into_json()?;

    match res.next.as_deref() {
        None => Ok((res.items, true)),
        Some("null") => Ok((res.items, true)),
        Some(..) => Ok((res.items, false)),
    }
}

pub fn create_playlist(name: &str, token: &Token) -> Result<Uri, UreqOrJSONError> {
    // Blocked: need to have a way to get User ID
    unimplemented!()
}
