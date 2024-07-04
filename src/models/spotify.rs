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
