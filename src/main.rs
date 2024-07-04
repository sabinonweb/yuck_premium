use dotenv::dotenv;
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};

use crate::cli::command_line;
use crate::downloader::process_playlist_download;
use crate::spotify::get_playlist_details;

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
    println!("creds: {:?}", creds);

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

    print!("url: {:?}", url);

    println!(
        "\n\n\n\n\naccess token {:?}",
        spotify_client.token.lock().await.unwrap()
    );

    spotify_client.prompt_for_token(&url).await.unwrap();
    println!(
        "\n\n\n\n\naccess token {:?}",
        spotify_client.token.lock().await.unwrap()
    );

    let mut cli_args = command_line().await;
    let uri_segments = cli_args.parse_uri();
    let playlist = get_playlist_details(uri_segments[4].to_string(), &spotify_client)
        .await
        .unwrap();
    process_playlist_download(playlist, &mut cli_args).await;
}
