use crate::models::cli::{Bitrate, Codec};
use crate::models::spotify::Spotify;
use clap::builder::PossibleValue;
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use std::u32;

#[derive(Debug, Clone)]
pub struct Config {
    pub id: String,
    pub spotify_type: Spotify,
    pub file_path: PathBuf,
    pub codec: Codec,
    pub bitrate: Bitrate,
    pub chunk: Option<u32>,
}

impl Config {
    pub fn parse_config(matches: &ArgMatches) -> Config {
        // clap checks for required flags during the entry, so it is safe to unwrap the value here
        let spotify_id: String = matches.get_one::<String>("spotify_id").unwrap().to_string();
        let file_path_prelim: String = matches.get_one::<String>("file_path").unwrap().to_string();
        let file_path = PathBuf::from(file_path_prelim);

        let spotify_type_prelim = matches.get_one::<String>("spotify");

        let spotify_type = match spotify_type_prelim {
            Some(s) => match s.to_owned().as_str() {
                "track" => Spotify::Track,
                "playlist" => Spotify::Playlist,
                "album" => Spotify::Album,
                _ => panic!("Spotify type not supported!"),
            },
            None => panic!("None of the types were provided!"),
        };

        let codec = match matches.get_one::<String>("codec") {
            Some(s) => match s.to_owned().as_str() {
                "mp3" => Codec::MP3,
                "mpa" => Codec::MPA,
                "flac" => Codec::Flac,
                "opus" => Codec::Opus,
                _ => panic!("File type is not supported!"),
            },
            None => panic!("None of the file types were provided!"),
        };

        let bitrate = match matches.get_one::<String>("bitrate") {
            Some(s) => match s.to_owned().as_str() {
                "worst" => Bitrate::Worst,
                "worse" => Bitrate::Worse,
                "poor" => Bitrate::Poor,
                "low" => Bitrate::Low,
                "medium" => Bitrate::Medium,
                "good" => Bitrate::Good,
                "high" => Bitrate::High,
                "best" => Bitrate::Best,
                _ => panic!("Bitrate not supported!"),
            },
            None => panic!("Bitrate not found!"),
        };

        let chunk = matches.get_one::<u32>("chunk").copied();

        Config {
            id: spotify_id,
            spotify_type,
            file_path,
            codec,
            bitrate,
            chunk,
        }
    }
}

pub async fn command_line() -> Config {
    let commands = parser();
    let matches = commands.get_matches();

    Config::parse_config(&matches)
}

pub fn parser() -> Command {
    Command::new("yuck_premium")
        .author("sabinonweb")
        .about("\n\n
            ------------------------------------------------Hawk Tuah in your spotify premium thing!----------------------------------------------
")
        .arg(
            Arg::new("spotify")
                .long("spotify")
                .value_name("spotify")
                .value_parser([
                    PossibleValue::new("track"),
                    PossibleValue::new("playlist"),
                    PossibleValue::new("album"),
                ])
                .help("Name of the spotify entity to download!")
                .required(true),
        )
        .arg(
            Arg::new("spotify_id")
                .long("spotify_id")
                .value_name("spotify_id")
                .help("ID of the spotify entity to download!")
                .required(true),
        )
        .arg(
            Arg::new("file_path")
                .long("path")
                .value_name("file_path")
                .value_parser([
                    PossibleValue::new("mp3"),
                    PossibleValue::new("mpa"),
                    PossibleValue::new("flac"),
                    PossibleValue::new("opus"),
                ])
                .help("Path where the audio file is to be downloaded!")
                .required(true),
        )
        .arg(
            Arg::new("codec")
                .long("codec")
                // .value_names(["mp3", "mpa", "flac", "opus"])
                .default_value("mp3")
                .help("Name or ID of the spotify to download")
                .required(true),
        )
        .arg(
            Arg::new("bitrate")
                .long("bitrate")
                .value_parser([
                    PossibleValue::new("worst"),
                    PossibleValue::new("worse"),
                    PossibleValue::new("poor"),
                    PossibleValue::new("low"),
                    PossibleValue::new("medium"),
                    PossibleValue::new("good"),
                    PossibleValue::new("high"),
                    PossibleValue::new("best"),
                ])
                .help("Name or ID of the spotify to download")
                .required(true),
        )
        .arg(
            Arg::new("chunk")
                .long("chunk")
                .action(ArgAction::Append)
                .help("Number of parallel downloads at a time")
                .required(true)
                .value_parser(value_parser!(u32)),
        )
    // .override_usage(
    //     "yuck_premium [OPTIONS] --spotify --spotify_id <ID> --file_path <FILE_PATH> --codec <CODEC> --bitrate <BITRATE>"
}
