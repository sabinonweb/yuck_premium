use log::error;
use youtube_dl::{SearchOptions, YoutubeDl};
use crate::{cli::Config, models::spotify::{SpotifyPlaylist, SpotifyTrack}};

fn create_query(spotify_song: SpotifyTrack) -> String {
    format!("{} - {:?}", spotify_song.song.name, spotify_song.song.artists.join(", "))
}

pub async fn download_song(spotify_song: SpotifyTrack, cli_args: &mut Config) -> bool {
    let file_format = format!("{}.{}", spotify_song.song.name, cli_args.codec);
    
    cli_args.file_path.push(file_format);

    let _ = match cli_args.file_path.parent() {
        None => {
            error!("No parent directory found!");
            return false;
        }
        Some(v) => v
    };
    
    let query = create_query(spotify_song.clone());

    let mut yt_client = YoutubeDl::search_for(&SearchOptions::youtube(query));

    if let Err(err) = yt_client
        .extract_audio(true)
        .extra_arg("--audio-format")
        .extra_arg(cli_args.codec.to_string())
        .extra_arg("--audio-quality")
        .extra_arg(cli_args.bitrate.to_string())
        .download_to_async(cli_args.clone().file_path)
        .await {
        error!("Error while downloading the song {} by {}", spotify_song.song.name, spotify_song.song.artists.as_slice().join(", "));
        error!("{:?}", err);
        return false;
    }
    
    println!("Download Complete!");

    true
}

pub async fn download_playlist(spotify_playlist: SpotifyPlaylist, cli_args: &mut Config) -> bool {
    let parallel_downloads = if cli_args.chunk.is_some() {
        cli_args.chunk.unwrap()
    } else {
        10
    };

    let no_of_songs = spotify_playlist.number_of_songs;

    if no_of_songs / parallel_downloads

    true
}
