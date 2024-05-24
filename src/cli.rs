use rspotify::AuthCodeSpotify;

use crate::spotify::from_id;

#[derive(Debug)]
pub struct Config {
    pub uri: String,
    pub file_type: String,
}

impl Config {
    fn parse_config(args: &[String]) -> Config {
        let uri = args[1].clone();
        let file_type = args[2].clone();
        let config = Config { uri, file_type };
        config.parse_uri();

       config 
    }

    fn parse_uri(&self) -> Vec<&str> {
        let uri_segments: Vec<&str> = self.uri.split("/").collect(); 
        uri_segments
    }
}

pub async fn command_line(spotify_client: &AuthCodeSpotify) {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::parse_config(&args);
    let uri_segments = config.parse_uri();

    from_id(uri_segments[4].to_owned(), spotify_client).await;

    println!("{:?}", config);
}
