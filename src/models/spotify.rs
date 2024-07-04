use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct SpotifyTrack {
    pub name: String,
    pub artists: Vec<String>,
    pub album_name: String,
    pub album_cover: String,
    pub disc_number: i32,
    pub track_number: u32,
}

#[derive(Clone, Debug)]
pub struct SpotifyAlbum {
    pub name: String,
    pub tracks: Vec<SpotifyTrack>,
    pub number_of_songs: u32,
    pub cover_url: Option<String>,
}

#[derive(Clone, Debug)]
pub struct SpotifyPlaylist {
    pub name: String,
    pub number_of_songs: u32,
    pub tracks: Vec<SpotifyTrack>,
    pub cover_url: Vec<String>,
}

pub enum Spotify {
    Album,
    Playlist,
    Track,
}

impl FromStr for Spotify {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "album" => Ok(Spotify::Album),
            "playlist" => Ok(Spotify::Playlist),
            "track" => Ok(Spotify::Track),
            _ => Err("Specification: {album, playlist, track}".to_string()),
        }
    }
}
