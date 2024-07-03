use crate::{
    cli::Config,
    models::spotify::{SpotifyAlbum, SpotifyPlaylist, SpotifyTrack},
};
use log::error;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::task::JoinHandle;
use youtube_dl::{SearchOptions, YoutubeDl};

pub const FILTER_LETTERS: &[char] = &['/', '<', '>', '"', ' ', '(', ')'];

fn create_query(spotify_song: SpotifyTrack) -> String {
    format!(
        "{} - {:?}",
        spotify_song.name,
        spotify_song.artists.join(", ")
    )
}

pub async fn download_song(
    spotify_song: SpotifyTrack,
    cli_args: Arc<Config>,
    file_path: PathBuf,
) -> bool {
    let file_format = format!("{}.{}", spotify_song.name, cli_args.codec);

    let _ = match cli_args.file_path.parent() {
        None => {
            error!("No parent directory found!");
            return false;
        }
        Some(v) => v,
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
        .await
    {
        error!(
            "Error while downloading the song {} by {}",
            spotify_song.name,
            spotify_song.artists.as_slice().join(", ")
        );
        error!("{:?}", err);
        return false;
    }

    println!("Download Complete!");

    true
}

pub async fn process_song_download(spotify_song: SpotifyTrack, cli_args: &mut Config) {
    let file_format = format!("{}.{}", spotify_song.name, cli_args.codec);
    cli_args.file_path.push(file_format);
    let cli_args = Arc::new(cli_args.clone());
    let file_path = cli_args.file_path.clone();
    download_song(spotify_song, cli_args, file_path).await;
}

pub async fn download_album_songs(
    songs: Vec<SpotifyTrack>,
    cli_args: Arc<Config>,
    file_path: PathBuf,
) -> bool {
    println!("songss: {:?}\n", songs);
    for song in songs.into_iter() {
        // println!("song: {:?}\n", song);

        // let file_format = format!("{}.{}", song.name, cli_args.codec);
        // file_path.push(file_format.clone());
        if download_song(song, cli_args.clone(), file_path.clone()).await {
            // println!("Downloading: {:?}", file_format.clone());
        }
    }
    true
}

pub async fn download_album_art(
    album_art_dir: &Path,
    link: Option<String>,
    spotify_album: &SpotifyAlbum,
) {
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
            } else {
                println!("Directory already exists");
            }
        }

        match File::create(directory) {
            Ok(mut file) => {
                println!("File created successfully!");
                file.write_all(&image).unwrap();
            }
            Err(err) => error!("File couldn't be created: {:?}", err),
        }
    }
}

pub async fn process_album_download(spotify_album: SpotifyAlbum, cli_args: &mut Config) -> bool {
    let mut parallel_downloads = if cli_args.chunk.is_some() {
        cli_args.chunk.unwrap()
    } else {
        10
    };

    download_album_art(
        &cli_args.file_path,
        spotify_album.cover_url.clone(),
        &spotify_album,
    )
    .await;

    cli_args.file_path.push(spotify_album.name);
    let file_path = cli_args.file_path.clone();
    println!("file_path:   {:?}", file_path);

    let cli_args = Arc::new(cli_args.clone());
    let no_of_songs = spotify_album.number_of_songs;
    let total_tasks = no_of_songs.div_ceil(parallel_downloads);

    if no_of_songs < parallel_downloads {
        parallel_downloads = no_of_songs;
    }

    let mut chunk: Vec<SpotifyTrack>;
    let mut handles = Vec::with_capacity(total_tasks as usize);

    for songs in spotify_album.tracks.chunks(parallel_downloads as usize) {
        chunk = songs.to_vec();
        let handle = tokio::spawn(download_album_songs(
            chunk,
            cli_args.clone(),
            file_path.clone(),
        ));

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    true
}

pub async fn download_playlist_songs(
    songs: Vec<SpotifyTrack>,
    cli_args: Arc<Config>,
    file_path: PathBuf,
) -> bool {
    for song in songs {
        tokio::spawn(download_playlist_songs_art(file_path.clone(), song.clone()))
            .await
            .unwrap();

        tokio::spawn(download_song(song, cli_args.clone(), file_path.clone()))
            .await
            .unwrap();
    }
    true
}

pub async fn download_playlist_songs_art(album_art_dir: PathBuf, song: SpotifyTrack) {
    let mut dir = album_art_dir.to_owned();

    let image = reqwest::get(song.album_cover).await.unwrap();
    if let Some(parent) = album_art_dir.parent() {
        if !parent.exists() {
            match create_dir_all(parent) {
                Ok(_) => println!("Directory created successfully!"),
                Err(err) => error!("Error while creating a directory: {:?}", err),
            }
        } else {
            println!("Directory already exists!");
        }
    }

    let song_name = song
        .name
        .chars()
        .filter(|c| !FILTER_LETTERS.contains(c))
        .collect::<String>();
    let file_format = format!("{}.jpeg", song_name);
    dir.push(file_format);

    let mut image_file = match File::create(dir.clone()) {
        Ok(file) => file,
        Err(err) => {
            error!("Error while creating a file: {:?}", err);
            return;
        }
    };
    println!("image_file: {:?}", image_file);
    image_file.write_all(&image.bytes().await.unwrap()).unwrap();
    println!("\ndirbrefore : {:?}\n", dir.clone());
    dir.pop();
    println!("\ndirafter : {:?}\n", dir);
}

pub async fn process_playlist_download(spotify_playlist: SpotifyPlaylist, cli_args: &mut Config) {
    if spotify_playlist.number_of_songs == 0 {
        error!("There's no song to download!\n");
        return;
    }

    cli_args.file_path.push(spotify_playlist.name);
    let file_path = cli_args.file_path.clone();

    let parallel_downloads = if let Some(chunk) = cli_args.chunk {
        chunk
    } else {
        10
    };

    let number_of_songs = spotify_playlist.number_of_songs;
    let no_of_tasks = number_of_songs.div_ceil(parallel_downloads);
    let mut chunk_of_songs: Vec<SpotifyTrack>;
    let mut handles: Vec<JoinHandle<bool>> = Vec::with_capacity(no_of_tasks as usize);
    let cli_args = Arc::new(cli_args.clone());

    for songs in spotify_playlist.tracks.chunks(parallel_downloads as usize) {
        chunk_of_songs = songs.to_vec();

        let handle = tokio::spawn(download_playlist_songs(
            chunk_of_songs,
            cli_args.clone(),
            file_path.clone(),
        ));
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
