use cli::parser;
use dotenv::dotenv;
// use downloader::progress_bar;
use log::error;
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};

use crate::cli::command_line;
use crate::downloader::{
    process_album_download, process_playlist_download, process_track_download,
};
use crate::models::spotify::Spotify;
use crate::spotify::{get_album_details, get_playlist_details, get_track_details};

mod cli;
mod downloader;
mod metadata;
mod models;
mod spotify;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // gets the user Credentials specified as RSPOTIFY_CLIENT_ID and RSPOTIFY_CLIENT_SECRET
    let client_id = std::env::var("RSPOTIFY_CLIENT_ID").unwrap();
    let client_secret = std::env::var("RSPOTIFY_CLIENT_SECRET").unwrap();

    let creds = Credentials::new(&client_id, &client_secret);

    // define the scopes needed for downloading
    let oauth = OAuth::from_env(scopes!(
        "playlist-read-private",
        "playlist-read-collaborative",
        "user-read-currently-playing",
        "playlist-modify-public",
        "playlist-modify-private",
        "user-library-read",
        "user-library-modify",
        "user-read-private"
    ))
    .unwrap();

    let mut spotify_client = AuthCodeSpotify::new(creds, oauth);
    spotify_client.config.token_cached = true;

    // obtaining the access token, returns the redirect uri
    // HTTP/1.1 302 FOUND
    // location: https://client.example.com/cb?code=authorizationcode&state=oauth
    let url = spotify_client.get_authorize_url(false).unwrap();

    spotify_client.prompt_for_token(&url).await.unwrap();
    parser();

    let mut cli_args = command_line().await;
    let spotify_id = &cli_args.id.to_owned();
    let spotify: Spotify = cli_args.spotify_type.clone();

    println!(
        r"__   __          _    ____                     _
 \ \ / /   _  ___| | _|  _ \ _ __ ___ _ __ ___ (_)_   _ _ __ ___
  \ V / | | |/ __| |/ / |_) | '__/ _ \ '_ ` _ \| | | | | '_ ` _ \
   | || |_| | (__|   <|  __/| | |  __/ | | | | | | |_| | | | | | |
   |_| \__,_|\___|_|\_\_|   |_|  \___|_| |_| |_|_|\__,_|_| |_| |_|
                                                                 "
    );

    // progress_bar(10);

    match spotify {
        Spotify::Album => {
            let album = match get_album_details(spotify_id.clone(), &spotify_client).await {
                Some(album) => album,
                None => {
                    error!("Details of album {} couldn't be fetched!", spotify_id);
                    return;
                }
            };
            process_album_download(album, &mut cli_args).await;
        }

        Spotify::Playlist => {
            let playlist = match get_playlist_details(spotify_id.clone(), &spotify_client).await {
                Some(playlist) => playlist,
                None => {
                    error!("Details of playlist {} couldn't be fetched!", spotify_id);
                    return;
                }
            };
            process_playlist_download(playlist, &mut cli_args).await;
        }

        Spotify::Track => {
            let track = match get_track_details(spotify_id.clone(), &spotify_client).await {
                Some(track) => track,
                None => {
                    error!("Details of track {} couldn't be fetched!", spotify_id);
                    return;
                }
            };
            process_track_download(track, &mut cli_args).await;
        }
    }
}
