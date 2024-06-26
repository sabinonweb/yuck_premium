use rspotify::{ 
    model::{AdditionalType, Country, Market}, prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth
};
use dotenv::dotenv;
// use crate::cli::command_line;
use crate::spotify::get_album_details;

// mod cli;
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

    get_album_details(String::from("2Ti79nwTsont5ZHfdxIzAm"), &spotify_client).await;
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
