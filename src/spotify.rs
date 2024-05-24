use rspotify::{clients::BaseClient, model::{ArtistId, FullArtist, Page, PlayableItem, PlaylistId, PlaylistItem, TrackId}, AuthCodeSpotify};
use log::error;

#[derive(Debug)]
pub struct Track {
    pub name: String,
    pub artists: Vec<String>,
    pub disc_number: i32,
    pub track_number: u32,
} 

pub struct SpotifyTrack {
    pub song: Track,
    pub album_name: String,
    pub cover_url: Option<String>,
}

pub struct SpotifyAlbum {
    pub name: String,
    pub tracks: Vec<Track>,
    pub number_of_songs: u32,
    pub cover_url: Option<String>,
}

pub struct SpotifyPlaylist {
    pub name: String,
    pub tracks: Vec<Track>,
    pub number_of_songs: u32,
    pub cover_url: Option<String>,
}

pub async fn from_id(spotify_id: String, client: &AuthCodeSpotify) {
     let id = match ArtistId::from_id(&spotify_id) {
        Ok(id) => Ok(id),
        Err(err) => Err(format!("Couldn't parse the given spotify id: {}", err)),
    }.unwrap();
    
    // GET request to the /artists/{id} api by sending an id
    // let artists = client.artist(id).await.unwrap();
    // println!("\n\nartists: {:?}\n\n\n", artists);
    get_playlists_details(spotify_id, client).await;
}

pub async fn get_track_details(spotify_id: String, client: &AuthCodeSpotify) -> Option<SpotifyTrack> {
    let id = match TrackId::from_id(spotify_id) {
        Ok(id) => Ok(id),
        Err(err) => Err(format!("Couldn't parse the given spotify id: {}", err)),
    }.unwrap();
    
    let track = client.track(id, None).await.unwrap();

    let song = Track {
        name: track.name,
        artists: track.artists.iter().map(|artist| artist.name.clone()).collect(),
        disc_number: track.disc_number,
        track_number: track.track_number,
    };
 
    Some(SpotifyTrack {
        song,
        album_name: track.album.name,
        cover_url: track.album.images.first().map(|image| image.url.clone()),
    })
}

pub async fn get_tracks_from_items(items: Vec<PlaylistItem>) {
    let mut tracks: Vec<Track> = Vec::new();

    for track in items {
       let song = Track {
            name: track.track.unwrap().name,
            artists: track.artists.iter().map(|artist| artist.name.clone()).collect(),
            disc_number: track.disc_number,
            track_number: track.track_number,
        };
    }
}

pub async fn get_playlists_details(spotify_id: String, client: &AuthCodeSpotify) -> Option<SpotifyPlaylist> {
    let id = match PlaylistId::from_id(spotify_id) {
        Ok(id) => Ok(id),
        Err(err) => Err(format!("Couldn't parse the given spotify id {}", err)),
    }.unwrap();

    let playlist = match client.playlist(id, None, None).await {
        Ok(playlist) => playlist,
        Err(err) => {
            error!("Couldn't fetch playlist data: {:?}", err);
            return None;
        }
    };

    let number_of_songs = playlist.tracks.items.len();
    println!("\n\nnumver {:?}\n\n", number_of_songs);
    
    for item in playlist.tracks.items {
        println!("\n\nitems: {:?}\n\n", item.track);
    }
        
    // for item in playlist.tracks.items {
        //println!("\n\n\nplaylist: {:?}\n\n", playlist.tracks);

    // }
    // let playlist = playlist.tracks.items;
    
    None
}
