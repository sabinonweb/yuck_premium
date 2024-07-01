use std::{path::PathBuf, sync::Arc};

use log::error;
use youtube_dl::{SearchOptions, YoutubeDl};
use crate::{cli::Config, models::spotify::{SpotifyPlaylist, SpotifyTrack, Track}};

fn create_query(spotify_song: Track) -> String {
    format!("{} - {:?}", spotify_song.name, spotify_song.artists.join(", "))
}

pub async fn download_song(spotify_song: Track, cli_args: &Config, mut file_path: PathBuf) -> bool {
    let file_format = format!("{}.{}", spotify_song.name, cli_args.codec);
    // file_path.push(file_format.clone()); 
    // cli_args.file_path.push(file_format.clone());
    // println!("file_path: {:?}", cli_args.file_path);

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
        .output_template(file_format.clone())
        .extra_arg("--audio-format")
        .extra_arg(cli_args.codec.to_string())
        .extra_arg("--audio-quality")
        .extra_arg(cli_args.bitrate.to_string())
        .download_to_async(file_path)
        .await {
        error!("Error while downloading the song {} by {}", spotify_song.name, spotify_song.artists.as_slice().join(", "));
        error!("{:?}", err);
        return false;
    }

    println!("Download Complete!");

    true
}

pub async fn process_song_download(spotify_song: Track, cli_args: &mut Config) {
    let file_format = format!("{}.{}", spotify_song.name, cli_args.codec);
    cli_args.file_path.push(file_format);
    let file_path = cli_args.file_path.clone();
    download_song(spotify_song, cli_args, file_path).await;
}

pub async fn process_playlist_download(spotify_playlist: SpotifyPlaylist, cli_args: &mut Config) -> bool {
    let mut parallel_downloads = if cli_args.chunk.is_some() {
        cli_args.chunk.unwrap()
    } else {
        10
    };

    cli_args.file_path.push(spotify_playlist.name);
    let mut file_path = cli_args.file_path.clone();
    println!("file_path:   {:?}", file_path);
    
    let cli_args = Arc::new(cli_args.clone());
    let no_of_songs = spotify_playlist.number_of_songs;
    let total_tasks = no_of_songs.div_ceil(parallel_downloads);

    // if no_of_songs % parallel_downloads == 0 {
    //     no_of_tasks = parallel_downloads;
    // } else {
    //     for ran
    // }
        
    if no_of_songs < parallel_downloads {
        parallel_downloads = no_of_songs;
    }

    let mut chunk: Vec<Track>;
    let mut handles = Vec::with_capacity(total_tasks as usize);

    for songs in spotify_playlist.tracks.chunks(parallel_downloads as usize) {
        chunk = songs.to_vec();
        let handle = tokio::spawn(
            download_playlist(chunk, cli_args.clone(), file_path.clone())
        );

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    true
}

pub async fn download_playlist(songs: Vec<Track>, cli_args: Arc<Config>, mut file_path: PathBuf) -> bool {
    println!("somgss: {:?}\n", songs);
    for song in songs.into_iter() {
        println!("song: {:?}\n", song);

        let file_format = format!("{}.{}", song.name, cli_args.codec);
        file_path.push(file_format.clone());
        if download_song(song, &cli_args, file_path.clone()).await {
            println!("Downloading: {:?}", file_format.clone());
        }
        file_path.pop();
    }
    return false;
}
