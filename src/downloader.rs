use std::{collections::HashSet, fs::{create_dir_all, File}, io::Write, path::{Path, PathBuf}, sync::Arc};

use log::{error, log};
use youtube_dl::{SearchOptions, YoutubeDl};
use crate::{cli::Config, models::spotify::{SpotifyAlbum, SpotifyPlaylist, SpotifyTrack, Track}};

pub const FILTER_LETTERS: &[char] = &['/', '<', '>', '"', ' '];

fn create_query(spotify_song: Track) -> String {
    format!("{} - {:?}", spotify_song.name, spotify_song.artists.join(", "))
}

pub async fn download_song(spotify_song: Track, cli_args: &Config, file_path: PathBuf) -> bool {
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

pub async fn process_album_download(spotify_album: SpotifyAlbum, cli_args: &mut Config) -> bool {
    let mut parallel_downloads = if cli_args.chunk.is_some() {
        cli_args.chunk.unwrap()
    } else {
        10
    };

     // = create_directory(&cli_args.file_path).expect("No directory found for album-art");
    download_album_art(&cli_args.file_path, spotify_album.cover_url.clone(), &spotify_album).await;
    // download_album_art(album_art, )

    cli_args.file_path.push(spotify_album.name);
    let file_path = cli_args.file_path.clone();
    println!("file_path:   {:?}", file_path);
    
    let cli_args = Arc::new(cli_args.clone());
    let no_of_songs = spotify_album.number_of_songs;
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

    for songs in spotify_album.tracks.chunks(parallel_downloads as usize) {
        chunk = songs.to_vec();
        let handle = tokio::spawn(
            download_album(chunk, cli_args.clone(), file_path.clone())
        );

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    true
}

pub async fn download_album(songs: Vec<Track>, cli_args: Arc<Config>, mut file_path: PathBuf) -> bool {
    println!("somgss: {:?}\n", songs);
    for song in songs.into_iter() {
        // println!("song: {:?}\n", song);

        
        let file_format = format!("{}.{}", song.name, cli_args.codec);
        file_path.push(file_format.clone());
        if download_song(song, &cli_args, file_path.clone()).await {
            // println!("Downloading: {:?}", file_format.clone());
        }
        file_path.pop();
    }
    return false;
}

pub async fn download_album_art(album_art_dir: &Path, link: Option<String>, spotify_album: &SpotifyAlbum) {
    let mut directory = album_art_dir.to_owned();
   
    if link.is_some() {
        let response = reqwest::get(link.unwrap()).await.unwrap();
        let image = response.bytes().await.expect("Data couldn't be read!");

        directory.push(spotify_album.name.clone());
       
        let name: String = spotify_album
            .name
            .chars()
            .filter(|c| !FILTER_LETTERS.contains(c))
            .collect();

        let img_format = format!("{}.jpeg", name);
        directory.push(img_format);

        if let Some(parent) = directory.parent() {
            if !parent.exists() {
                match create_dir_all(parent) {
                    Ok(_) => println!("Directory created successfully!"),
                    Err(err) => {
                        error!("Directory couldn't be created: {:?}", err);
                        return;
                    }
                }
            }
            else {
                println!("Directory already exists");
            }
        } 
        
        match File::create(directory) {
            Ok(mut file) => {
                println!("File created successfully!");
                file.write_all(&image).unwrap();
            }
            Err(err) => error!("File couldn't be created: {:?}", err)
        }
    }
}
