use futures::TryFutureExt;
use rspotify::{clients::BaseClient, model::{AlbumId, ArtistId, FullArtist, Page, PlayableItem, PlaylistId, PlaylistItem, TrackId}, AuthCodeSpotify};
use crate::models::spotify::{SpotifyAlbum, SpotifyPlaylist, SpotifyTrack, Track};

pub async fn get_track_details(spotify_id: String, client: &AuthCodeSpotify) -> Option<SpotifyTrack> {
    let id = match TrackId::from_id(spotify_id) {
        Ok(id) => Ok(id),
        Err(err) => Err(format!("Couldn't parse the given track id: {}", err)),
    }.unwrap();
   
    // returns a FullTrack
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

pub async fn get_album_details(spotify_id: String, client: &AuthCodeSpotify) -> Option<SpotifyAlbum> {
    let album_id = match AlbumId::from_id(spotify_id) {
        Ok(id) => Ok(id),
        Err(err) =>  Err(format!("Couldn't parse the given album id: {}", err)),
    }.unwrap();

    let album = client.album(album_id, None).await.map_err(|err| println!("\n\nError: {:?}", err));
    // println!("album tracks: {:?}", album.clone().unwrap().tracks.items);
    let mut tracks: Vec<Track> = Vec::with_capacity(album.clone().unwrap().tracks.total as usize);
    
    for track in album.clone().unwrap().tracks.items {
        tracks.push(Track { 
            name: track.name, 
            artists: track.artists.iter().map(|artist| artist.name.clone()).collect(),
            disc_number: track.disc_number,
            track_number: track.track_number
        });
    }

    Some(SpotifyAlbum {
        name: album.clone().unwrap().name,
        tracks,
        number_of_songs: album.clone().unwrap().tracks.total,
        cover_url: album.clone().unwrap().images.first().map(|image| image.url.clone()),
    }) 
}

pub fn who_loves_podcasts_anyways(playable_items: Vec<PlaylistItem>) -> Vec<Track> {
    let mut tracks: Vec<Track> = Vec::new();

    for track in playable_items {
        let song = if let Some(track) = track.track {
            track
        } else {
            continue;
        };

        let PlayableItem::Track(track) = song else { continue; }; 

        tracks.push(Track {
            name: track.name,
            artists: track.artists.iter().map(|artist| artist.name.clone()).collect(),
            disc_number: track.disc_number,
            track_number: track.track_number,
        });
    }

    tracks
}

pub async fn get_playlist_details(spotify_id: String, client: &AuthCodeSpotify) -> Option<SpotifyPlaylist> {
    let playlist_id = match PlaylistId::from_id(spotify_id) {
        Ok(id) => Ok(id),
        Err(err) =>  Err(format!("Couldn't parse the given playlist id: {}", err)),
    }.unwrap();

    let playlist = client.playlist(playlist_id, None, None)
        .await
        .map_err(|err| println!("Error while searching playlist of the given id!"))
        .unwrap();
        
    let tracks = who_loves_podcasts_anyways(playlist.tracks.items);
    let mut cover_url: Vec<String> = Vec::new();

    for image in playlist.images {
        println!("\nURL: {:?}\n", image.url);
        cover_url.push(image.url);
    } 
    
    Some(SpotifyPlaylist { 
        name: playlist.name, 
        number_of_songs: playlist.tracks.total, 
        tracks,
        cover_url,
    })
}
