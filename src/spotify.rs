use rspotify::{
    model::{AdditionalType, Country, Market},
    prelude::*,
    scopes, AuthCodeSpotify, Credentials, OAuth,
};

async fn spotify() {
    // gets the user Credentials specified as RSPOTIFY_CLIENT_ID and RSPOTIFY_CLIENT_SECRET
    let creds = Credentials::from_env().unwrap();

    // define the scopes needed for downloading 
    let oauth = OAuth::from_env(
        scopes!(
            "playlist-read-private", "playlist-read-collaborative", "playlist-modify-public", "playlist-modify-private", "user-library-read", "user-library-modify", "user-read-private"
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

    let market = Market::Country(Country::Spain);
    let additional_types = [AdditionalType::Episode];
    let artists = spotify
        .current_playing(Some(market), Some(&additional_types))
        .await;

    println!("Response: {artists:?}");
} 
