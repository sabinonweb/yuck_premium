pub struct Track {
    pub name: String,
    pub artists: Vec<String>,
    pub disc_number: i32,
    pub track_number: u32,
} 

pub struct SpotifyTrack {
    pub song: Track,
    pub album_name: String,
}

pub struct SpotifyAlbum {
    pub name: String,
    pub tracks: Vec<Track>,
    pub number_of_songs: u32,
}

pub struct SpotifyPlaylist {
    pub name: String,
    pub tracks: Vec<Track>,
    pub number_of_songs: u32,
}
