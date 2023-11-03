use std::cmp::Ordering;

use clap::ValueEnum;

use crate::{commands::auth, spotify::{self, types::TrackObject, types::Uri}, util};
use spotify::types;
use crate::commands::playlist::sort::parsed_release_date::ParsedReleaseDate;

#[derive(ValueEnum, Clone, Copy, Debug)]
#[non_exhaustive]
pub enum SortMethod {
    AlbumRelease,
    TrackNumber,
}

impl SortMethod {
    pub fn cmp(&self, a: &types::TrackObject, b: &types::TrackObject) -> Ordering {
        match self {
            Self::AlbumRelease => self.cmp_album_release(a, b),
            Self::TrackNumber => self.cmp_track_number(a, b),
        }
    }

    fn cmp_album_release(&self, a: &types::TrackObject, b: &types::TrackObject) -> Ordering {
        let a_rd = a
            .album.as_ref()
            .map(|album| album.release_date.as_str())
            .ok_or(Err::<ParsedReleaseDate, String>("No album".to_owned()))
            .map(ParsedReleaseDate::try_from);
        let b_rd = b
            .album.as_ref()
            .map(|album| album.release_date.as_str())
            .ok_or(Err::<ParsedReleaseDate, String>("No album".to_owned()))
            .map(ParsedReleaseDate::try_from);

        match (a_rd, b_rd) {
            (Err(_), Err(_)) => Ordering::Equal,
            (Err(_), Ok(_)) => Ordering::Less,
            (Ok(_), Err(_)) => Ordering::Greater,
            (Ok(a), Ok(b)) => a.partial_cmp(&b).unwrap_or(Ordering::Equal)
        }
    }

    fn cmp_track_number(&self, a: &types::TrackObject, b: &types::TrackObject) -> Ordering {
        let a_tn = a.track_number;
        let b_tn = b.track_number;

        a_tn.cmp(&b_tn)
    }
}

pub fn run(path: std::path::PathBuf, playlist: &str, methods: &Vec<SortMethod>) {
    // Token acquisition
    let config = util::read_config(&path);
    let token = auth::refresh(&config);
    if token.is_err() {
        println!("Failed to auth: {:?}", token.unwrap_err());
        return;
    }
    let token = token.unwrap();

    let tracks = spotify::playlist::get_playlist_items(playlist, &token);
    if tracks.is_err() {
        println!("Failed to get playlist: {:?}", tracks.unwrap_err());
        return;
    }
    let tracks = tracks.unwrap();

    let mut chunks: Vec<Vec<TrackObject>> = vec![tracks];
    for method in methods {
        chunks = run_sort_round_by(chunks, *method);
    }

    let uris_ordered: Vec<Uri> = chunks.into_iter().flatten().map(|t| t.uri).collect();

    let res = spotify::playlist::set_playlist(playlist, &uris_ordered, &token);
    match res {
        Err(e) => {
            println!("Failed to set playlist:\n{:?}", e);
        }
        Ok(_) => {
            println!("Sorting complete.");
        }
    }
}

fn run_sort_round_by(chunks: Vec<Vec<TrackObject>>, method: SortMethod) -> Vec<Vec<TrackObject>> {
    let mut out: Vec<Vec<TrackObject>> = Vec::new();
    for mut chunk in chunks {
        chunk.sort_unstable_by(|a,b| method.cmp(a,b));
        let mut new: Vec<TrackObject> = vec![chunk.remove(0)];
        while !chunk.is_empty() {
            match method.cmp(new.last().unwrap(), chunk.first().unwrap()) {
                Ordering::Equal => {
                    new.push(chunk.remove(0))
                },
                _ => {
                    out.push(new);
                    new = vec![chunk.remove(0)];
                }
            }
        }
        out.push(new);
    }

    // println!("sort.rs:110\n{:?}\n-----", out);
    out
}
