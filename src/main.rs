// use reqwest::{header::{HeaderValue, CONTENT_LENGTH, RANGE}, Client, StatusCode};
// use std::{fs::File, str::FromStr};
//
// mod spotify;
//
// // const YOUTUBE: &str = "https://www.youtube.com/watch?v=De97zQi5rzc&list=PLwooShOrg79yVGfA9OcjATf9EjEvi4bQu";
//
// #[derive(Debug)]
// struct PartialRange {
//     start: u64,
//     end: u64,
//     buffer: u32,
// }
//
// impl PartialRange {
//     fn new(start: u64, end: u64, buffer: u32) -> Result<Self, String> {
//         if buffer == 0 {
//             Err(format!("Buffer should be greater than zero"))
//         } else {
//             Ok(PartialRange {
//                 start,
//                 end,
//                 buffer,
//             })
//         }
//     } 
// } 
//
// impl Iterator for PartialRange {
//     type Item = HeaderValue;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.start > self.end {
//             None
//         } else {
//             let prev_start = self.start;
//             self.start += std::cmp::min(self.buffer as u64, self.end - self.start + 1);
//             Some(HeaderValue::from_str(&format!("Range={}-{}", prev_start, self.start - 1)).expect("Expected &str, found incompatible type"))
//         }
//     } 
// }
//
// #[tokio::main]
// async fn main() -> Result<(), String> {
//     let url = "https://www.rust-lang.org/logos/rust-logo-16x16.png";
//     const CHUNK_SIZE: u32 = 10240;
//
//     let client = Client::new();
//     let response = client
//         .head(url)
//         .send()
//         .await
//         .unwrap();
//
//     let length = response
//         .headers()
//         .get(CONTENT_LENGTH)
//         .ok_or("content-length field not found in the response")
//         .unwrap();
//
//     let length = u64::from_str(length.to_str().expect("Expected HeaderValue")).unwrap();
//
//     for range in PartialRange::new(0, length - 1, CHUNK_SIZE).unwrap() {
//         println!("Range: {:?}", range);
//         let response = client
//             .get(url)
//             .header(RANGE, range)
//             .send()
//             .await
//             .unwrap();
//
//         if !(response.status() == StatusCode::PARTIAL_CONTENT || response.status() == StatusCode::OK) {
//             return Err(format!("Unexpected server response: {:?}", response.status()));
//         }
//         let mut file = File::create("download.png").unwrap();
//         //std::io::copy(&mut response, &mut file);
//         //println!("Response: {:?}", response);
//
//         let response_text = response.text().await.unwrap();
//         std::io::copy(&mut response_text.as_bytes(), &mut file).unwrap();
//
//         println!("finished");
//
//         println!("{:?}", std::cmp::min(10240, 10240));
//     }
//     Ok(())
// }
//
//

use rspotify::{
    model::{AdditionalType, Country, Market},
    prelude::*,
    scopes, AuthCodeSpotify, Credentials, OAuth,
};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
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
