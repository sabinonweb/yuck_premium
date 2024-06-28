use std::{fs::File, path::Path};

use rspotify::{ 
    model::{AdditionalType, Country, Market}, prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth
};
use dotenv::dotenv;
use youtube_dl::{SearchOptions, YoutubeDl};
// use crate::cli::command_line;
use crate::spotify::get_track_details;
use crate::cli::command_line;
use crate::downloader::download_song;

mod cli;
mod downloader;
mod spotify;
mod models;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // gets the user Credentials specified as RSPOTIFY_CLIENT_ID and RSPOTIFY_CLIENT_SECRET
    let client_id = std::env::var("RSPOTIFY_CLIENT_ID").unwrap();
    let client_secret = std::env::var("RSPOTIFY_CLIENT_SECRET").unwrap();

    let creds = Credentials::new(&client_id, &client_secret);
    println!("creds: {:?}", creds);

    // define the scopes needed for downloading 
    let oauth = OAuth::from_env(
        scopes!(
            "playlist-read-private", "playlist-read-collaborative", "user-read-currently-playing", "playlist-modify-public", "playlist-modify-private", "user-library-read", "user-library-modify", "user-read-private"
        )
    )
    .unwrap();
   
    let mut spotify_client = AuthCodeSpotify::new(creds, oauth);
    spotify_client.config.token_cached = true;

    // obtaining the access token, returns the redirect uri
    // HTTP/1.1 302 FOUND
    // location: https://client.example.com/cb?code=authorizationcode&state=oauth 
    let url = spotify_client.get_authorize_url(false).unwrap();
   
    print!("url: {:?}", url);
    //let binding = spotify_client.get_token();
    //let token = binding.lock().await.unwrap();

    println!("\n\n\n\n\naccess token {:?}", spotify_client.token.lock().await.unwrap());
    // println!("\n\n\n\ntoken: {:?}\n\n\n", token);
    
    // this prompt for taken makes request with the given authorization code and if everything goes
    // well access token is sent bacl
    spotify_client.prompt_for_token(&url).await.unwrap();
    println!("\n\n\n\n\naccess token {:?}", spotify_client.token.lock().await.unwrap());
    
    let mut cli_args = command_line().await;
    let uri_segments = cli_args.parse_uri();
    let track = get_track_details(uri_segments[4].to_string(), &spotify_client).await.unwrap();
    println!("\n\nartists' name: {:?}", track.song.artists.join(","));
    download_song(track, &mut cli_args).await;

    // let mut yt_client = YoutubeDl::search_for(&SearchOptions::youtube("The Elements - Indriya"));
    // println!("yt_Clienet: {:?}", yt_client);
    // match yt_client.extra_arg("mp3").extract_audio(true).download_to("./") {
    //     Ok(_) => println!("Successful!"),
    //     Err(err) => println!("Not found: {:?}", err)
    // }
    //
     // let playing = spotify_client
 //     .current_playing(Some(Market::Country(Country::Nepal)), Some(&[AdditionalType::Track]))
 //     .await
 //     .unwrap()
 //     .unwrap()
 //     .item
 //     .unwrap()
 //     .id().unwrap();
 // println!("Response: {:?} ", playing);
 //
 // command_line(&spotify_client).await;



 //println!("Response: {artists:?}");
}
