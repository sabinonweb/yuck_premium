use crate::models::cli::{Bitrate, Codec};
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Clone)]
pub struct Config {
    pub uri: String,
    pub file_path: PathBuf,
    pub codec: Codec,
    pub bitrate: Bitrate,
    pub chunk: Option<u32>,
}

impl Config {
    pub fn parse_config(args: &[String]) -> Config {
        let uri = args[1].clone();
        let file_path = args[2].clone().into();

        let codec = args[3].parse().unwrap();

        let bitrate = args[4].parse().unwrap();
        let chunk = match u32::from_str(&args[4][..]) {
            Ok(value) => Some(value),
            Err(_) => None,
        };
        println!("codec : {:?}, bitrate : {:?}", codec, bitrate);

        let config = Config {
            uri,
            file_path,
            codec,
            bitrate,
            chunk,
        };
        config.parse_uri();

        config
    }

    pub fn parse_uri(&self) -> Vec<&str> {
        let uri_segments: Vec<&str> = self.uri.split("/").collect();
        uri_segments
    }
}

pub async fn command_line() -> Config {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::parse_config(&args);
    let uri_segments = config.parse_uri();
    println!("{:?}", config);

    config
    // from_id(uri_segments[4].to_owned(), spotify_client).await;
}
