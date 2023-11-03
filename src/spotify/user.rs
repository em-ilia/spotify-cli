use serde::Deserialize;

use crate::spotify::types::{Token, TrackObject, UserId, UserProfile};
use crate::util::UreqOrJSONError;

pub fn get_user_id(token: &Token) -> Result<UserId, UreqOrJSONError> {
    Ok(get_user_profile(token)?.id)
}

pub enum TopTerm {
    Short,
    Medium,
    Long,
}

impl std::fmt::Display for TopTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Short => write!(f, "short_term"),
            Self::Medium => write!(f, "medium_term"),
            Self::Long => write!(f, "long_term"),
        }
    }
}

#[derive(Debug, Deserialize)]
struct GetUserTopTracksRes {
    items: Vec<TrackObject>,
    total: u16,
}

pub fn get_user_top_tracks(
    token: &Token,
    term: &TopTerm,
    num: u16,
) -> Result<Vec<TrackObject>, UreqOrJSONError> {
    let mut tracks: Vec<TrackObject> = Vec::with_capacity(num as usize);

    while tracks.len() < num as usize {
        let res = ureq::get(&(crate::spotify::BASE_URL.to_owned() + "/me/top/tracks"))
            .set("Authorization", &("Bearer ".to_owned() + &token.0))
            .query("limit", "50")
            .query("offset", &tracks.len().to_string())
            .query("time_range", &term.to_string())
            .call()?
            .into_json::<GetUserTopTracksRes>()?;

        tracks.extend(res.items);
        if tracks.len() >= res.total as usize {
            break
        }
    }

    Ok(tracks.into_iter().take(num as usize).collect())
}

fn get_user_profile(token: &Token) -> Result<UserProfile, UreqOrJSONError> {
    Ok(ureq::get(&(crate::spotify::BASE_URL.to_owned() + "/me"))
        .set("Authorization", &("Bearer ".to_owned() + &token.0))
        .call()?
        .into_json::<UserProfile>()?)
}
