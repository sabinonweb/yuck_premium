use rspotify::{
    clients::BaseClient, model::{
        image::Image, AlbumId, FullTrack, PlayableItem, PlaylistId, PlaylistItem, SimplifiedTrack, TrackId
    }, 
    AuthCodeSpotify};
use crate::models::spotify::{SpotifyAlbum, SpotifyPlaylist, SpotifyTrack};

pub async fn get_track_details(spotify_id: String, client: &AuthCodeSpotify) -> Option<SpotifyTrack> {
    let id = match TrackId::from_id(spotify_id) {
        Ok(id) => Ok(id),
        Err(err) => Err(format!("Couldn't parse the given track id: {}", err)),
    }.unwrap();
   
    // returns a FullTrack
    let track = client.track(id, None).await.unwrap();
    
    // filter the images and return the image with dimensions 640 * 640
    let album_cover = get_album_cover_url(&track);
    
    let song = SpotifyTrack {
        name: track.name,
        album_name: track.album.name,
        album_cover,
        //track.album.images
        //     .iter()
        //     .filter(|image| ), 
        artists: track.artists.iter().map(|artist| artist.name.clone()).collect(),
        disc_number: track.disc_number,
        track_number: track.track_number,
    };
    Some(song) 
}

fn get_album_cover_url(track: &FullTrack) -> String {
    let album_cover_uri = track
        .album
        .images
        .iter()
        .filter(|image| image.width.clone() == Some(640) && image.height.clone() == Some(640))
        .cloned()
        .collect::<Vec<Image>>();

    let mut album_cover = String::new();

    for image in album_cover_uri {
        album_cover = image.url.clone();
        break;
    }
    
    album_cover
}

fn get_album_cover_url_for_simplified_track(track: &SimplifiedTrack) -> String {
    let album_cover_uri = track
        .clone()
        .album
        .unwrap()
        .images
        .iter()
        .filter(|image| image.width.clone() == Some(640) && image.height.clone() == Some(640))
        .cloned()
        .collect::<Vec<Image>>();

    let mut album_cover = String::new();

    for image in album_cover_uri {
        album_cover = image.url.clone();
        break;
    }
    
    album_cover
}

pub async fn get_album_details(spotify_id: String, client: &AuthCodeSpotify) -> Option<SpotifyAlbum> {
    let album_id = match AlbumId::from_id(spotify_id) {
        Ok(id) => Ok(id),
        Err(err) =>  Err(format!("Couldn't parse the given album id: {}", err)),
    }.unwrap();

    let album = client.album(album_id, None).await.map_err(|err| println!("\n\nError: {:?}", err));
    // println!("album tracks: {:?}", album.clone().unwrap().tracks.items);
    let mut tracks: Vec<SpotifyTrack> = Vec::with_capacity(album.clone().unwrap().tracks.total as usize);
    
    for track in album.clone().unwrap().tracks.items {
        let album_cover = get_album_cover_url_for_simplified_track(&track);
        tracks.push(SpotifyTrack { 
            name: track.name, 
            artists: track.artists.iter().map(|artist| artist.name.clone()).collect(),
            disc_number: track.disc_number,
            track_number: track.track_number,
            album_cover,
            album_name: album.clone().unwrap().name
        });
    }

    Some(SpotifyAlbum {
        name: album.clone().unwrap().name,
        tracks,
        number_of_songs: album.clone().unwrap().tracks.total,
        cover_url: album.clone().unwrap().images.first().map(|image| image.url.clone()),
    }) 
}

pub fn who_loves_podcasts_anyways(playable_items: Vec<PlaylistItem>) -> Vec<SpotifyTrack> {
    let mut tracks: Vec<SpotifyTrack> = Vec::new();

    for track in playable_items {
        let song = if let Some(track) = track.track {
            track
        } else {
            continue;
        };

        let PlayableItem::Track(track) = song else { continue; };
        let album_cover = get_album_cover_url(&track);

        tracks.push(SpotifyTrack {
            name: track.name,
            artists: track.artists.iter().map(|artist| artist.name.clone()).collect(),
            disc_number: track.disc_number,
            track_number: track.track_number,
            album_cover,
            album_name:track.album.name,
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
