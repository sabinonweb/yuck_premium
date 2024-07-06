use lofty::config::WriteOptions;
use lofty::picture::{Picture, PictureType};
use lofty::prelude::*;
use lofty::probe::Probe;
use lofty::tag::Tag;
use log::{error, info};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::models::spotify::SpotifyTrack;

pub fn add_metadata(spotify_song: SpotifyTrack, album_art_dir: PathBuf, file_path: PathBuf) {
    println!("album_dir: {:?}, file_path: {:?}", album_art_dir, file_path);
    println!("\n\nADD METADATA CALLED\n\n");
    // reads the file type from the path and open the File as File::open
    let probed_file = match Probe::open(&file_path) {
        Err(err) => {
            error!(
                "Error occured while reading the file_path {:?}: {}",
                file_path, err
            );
            return;
        }

        Ok(probe) => probe,
    };

    let mut tagged_file = match probed_file.read() {
        Ok(file) => {
            println!("Tagged FIle\n");
            file
        }
        Err(err) => {
            error!("Error occured while reading the probe: {}", err);
            return;
        }
    };

    // primary_tag_mut returns the primary tag type of the file
    let tag = match tagged_file.primary_tag_mut() {
        Some(tag) => tag,
        None => {
            // first tag returns the first tag of the file
            if let Some(tag) = tagged_file.first_tag_mut() {
                tag
            } else {
                let tag_type = tagged_file.primary_tag_type();

                println!(
                    "WARN: No tags found, creating a new tag of type: {:?}",
                    tag_type
                );
                tagged_file.insert_tag(Tag::new(tag_type));

                tagged_file.primary_tag_mut().unwrap()
            }
        }
    };

    let artist = spotify_song.artists.join(", ");
    tag.set_artist(artist);
    tag.set_title(spotify_song.name);
    tag.set_album(spotify_song.album_name);
    tag.set_disk(spotify_song.disc_number as u32);
    tag.set_track(spotify_song.track_number);

    let image_file = match File::open(album_art_dir.clone()) {
        Ok(file) => file,
        Err(err) => {
            error!("Error while opening the file {:?}: {}", album_art_dir, err);
            return;
        }
    };

    let mut image = BufReader::new(image_file);

    let mut picture = match Picture::from_reader(&mut image) {
        Ok(pic) => pic,
        Err(err) => {
            error!("Error occured while reading picture from BufRead: {}", err);
            return;
        }
    };
    picture.set_pic_type(PictureType::CoverFront);
    tag.push_picture(picture);

    match tag.save_to_path(file_path.clone(), WriteOptions::default()) {
        Ok(_) => {
            println!("\nFile saved\n");
            info!("Tag saved to the path: {:?}", file_path);
        }
        Err(err) => {
            error!(
                "Error occured while saving tag to the path {:?}: {}",
                file_path, err
            );
            return;
        }
    }

    println!("End of add_Data");
}

pub fn check_metadata(file_path: &PathBuf) {
    println!("\n\nCHECK METADATA CALLED\n\n");
    let probed_file = match Probe::open(file_path) {
        Ok(probe) => probe,
        Err(err) => {
            error!(
                "Error occured while reading the file_path {:?}: {}",
                file_path, err
            );
            return;
        }
    };

    let tagged_file = match probed_file.read() {
        Ok(file) => file,
        Err(err) => {
            error!("Error occured while reading the probe: {}", err);
            return;
        }
    };

    let tag = match tagged_file.primary_tag() {
        Some(tag) => tag,
        None => tagged_file.first_tag().expect("No tag found!"),
    };

    println!("                      ------------Audio Information----------");
    println!(
        "                      Title: {}",
        tag.title().as_deref().unwrap_or("None")
    );
    println!(
        "                      Artist: {}",
        tag.artist().as_deref().unwrap_or("None")
    );
    println!(
        "                      Album: {}",
        tag.album().as_deref().unwrap_or("None")
    );
    println!(
        "                      Disk: {}",
        tag.disk().expect("Disk number not found!")
    );
    println!(
        "                      Track: {}",
        tag.track().expect("Track number not found!")
    );
}
