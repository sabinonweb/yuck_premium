use colored::Colorize;
use lofty::config::WriteOptions;
use lofty::prelude::*;
use lofty::probe::Probe;
use log::error;
use serde::Deserialize;
use std::{fs, path::Path};

use crate::models::spotify::SpotifyTrack;

const LRCLIB_GET: &str = "https://lrclib.net/api/get";

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lyrics {
    pub plain_lyrics: Option<String>,
    pub synced_lyrics: Option<String>,
}

// Looks up lyrics on LRCLIB. Returns None when nothing is found or the request fails.
pub async fn fetch_lyrics(song: &SpotifyTrack) -> Option<Lyrics> {
    let duration = (song.duration_ms / 1000).to_string();
    let artist = song.artists.first().cloned().unwrap_or_default();

    let response = reqwest::Client::new()
        .get(LRCLIB_GET)
        .header("User-Agent", "yuck_premium (https://github.com/sabinonweb/yuck_premium)")
        .query(&[
            ("track_name", song.name.as_str()),
            ("artist_name", artist.as_str()),
            ("album_name", song.album_name.as_str()),
            ("duration", duration.as_str()),
        ])
        .send()
        .await
        .ok()?;

    if !response.status().is_success() {
        return None;
    }

    response.json::<Lyrics>().await.ok()
}

// Fetches lyrics for `song` and saves them next to / inside the audio file at `song_path`:
// synced lyrics go to a sidecar `.lrc` file, plain lyrics are embedded in the tag.
// Prints a yellow warning when nothing is found; never fails the download.
pub async fn save_lyrics(song: &SpotifyTrack, song_path: &Path) {
    let lyrics = match fetch_lyrics(song).await {
        Some(l) if l.plain_lyrics.is_some() || l.synced_lyrics.is_some() => l,
        _ => {
            println!(
                "{}",
                format!("WARN: No lyrics found for {}", song.name).yellow()
            );
            return;
        }
    };

    if let Some(synced) = &lyrics.synced_lyrics {
        let lrc_path = song_path.with_extension("lrc");
        if let Err(err) = fs::write(&lrc_path, synced) {
            error!("Couldn't write {:?}: {}", lrc_path, err);
        }
    }

    if let Some(plain) = &lyrics.plain_lyrics {
        embed_lyrics(song_path, plain);
    }
}

fn embed_lyrics(song_path: &Path, plain: &str) {
    let mut tagged_file = match Probe::open(song_path).and_then(|p| p.read()) {
        Ok(f) => f,
        Err(err) => {
            error!("Couldn't read {:?} for lyrics: {}", song_path, err);
            return;
        }
    };

    let Some(tag) = tagged_file.primary_tag_mut() else {
        error!("No tag in {:?} to add lyrics to", song_path);
        return;
    };

    tag.insert_text(ItemKey::Lyrics, plain.to_string());

    if let Err(err) = tag.save_to_path(song_path, WriteOptions::default()) {
        error!("Couldn't save lyrics to {:?}: {}", song_path, err);
    }
}
