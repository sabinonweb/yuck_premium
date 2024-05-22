use rspotify::{
    model::{AdditionalType, Country, Market},
    prelude::*,
    scopes, AuthCodeSpotify, Credentials, OAuth,
};
use dotenv::dotenv;
use crate::cli::command_line;

mod cli;

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
    
    let spotify = AuthCodeSpotify::new(creds, oauth);

    // obtaining the access token, returns the redirect uri
    // HTTP/1.1 302 FOUND
    // location: https://client.example.con/cb?code=authorizationcode&state=same_as_sent_in_request
    let url = spotify.get_authorize_url(false).unwrap();
    print!("url: {:?}", url);

    spotify.prompt_for_token(&url).await.unwrap();

    command_line();


    let market = Market::Country(Country::Spain);
    let additional_types = [AdditionalType::Episode];
    let artists = spotify
        .current_playing(Some(market), Some(&additional_types))
        .await;

    println!("Response: {artists:?}");
}
