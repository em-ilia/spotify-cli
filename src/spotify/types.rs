// Is it plagiarism if it's my own code? No.
// Stolen from cool-spotify-blend, lovingly modified

use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub struct PlaylistTrackObject {
    pub track: TrackObject
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub struct TrackObject {
    pub album: Option<AlbumObject>,
    pub artists: Option<Vec<ArtistObject>>,
    pub href: String,
    pub id: String,
    pub name: String,
    pub uri: Uri,
    pub track_number: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub struct AlbumObject {
    pub album_type: String,
    pub total_tracks: u32,
    pub href: String,
    pub id: String,
    pub name: String,
    pub release_date: String,
    pub uri: Uri,
    pub genres: Option<String>,
    pub images: Vec<ImageObject>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub struct ArtistObject {
    pub href: String,
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Uri(pub String);

impl Uri {
    pub fn _get_suffix(&self) -> String {
        self.0
            .split(':')
            .nth(2)
            .expect("URI should have the correct format")
            .to_string()
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpotifyId(pub String);

#[derive(Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub struct UserProfile {
    pub id: UserId,
    pub display_name: Option<String>,
    pub images: Option<Vec<ImageObject>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub struct ImageObject {
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Token(pub String);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserId(pub String);
