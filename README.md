# yuck_premium

yuck_premium is a command-line application designed to download music tracks, playlists, and albums from spotify. Users can specify the content they wish to download using unique identifiers, along with desired bitrate, codec, and destination path for saving the files. This tool aims to provide a seamless and efficient way to archive and enjoy music offline.

# Setup

To use yuck_premium, you need two things: FFmpeg and a Spotify developer account.

### Steps to Create a Spotify Developer Account:

1. **Access Spotify Developer Dashboard**: Go to the [Spotify Developer Dashboard](https://developer.spotify.com/dashboard/).
2. **Login and Create an App**: Log in with your Spotify credentials and click on "Create an App".
3. **App Details**: Enter a name and description for your app, check the required checkbox, and proceed.
4. **Retrieve Client Credentials**: You will now have access to your Client ID. Click on "Show Client Secret" to get your Client Secret.
5. **Set Redirect URI**: Click on "Edit Settings" and add a redirect URI in the "Redirect URIs" section. A common choice is `http://localhost:6375/callback`.

### Define Environment Variables

Set the following environment variables in your system:

- `SPOTIPY_CLIENT_ID`
- `SPOTIPY_CLIENT_SECRET`
- `SPOTIPY_REDIRECT_URI`

### Initial Run

The first time you run yuck_premium, a popup window in your browser will prompt you to authorize the app you just created in the Spotify Developer Dashboard. Accept the request and close the window. If you've already granted access to the app, the window will automatically close.

# Usage

### Single Track Download

```sh
cargo run -- --spotify track --spotify_id <track_id> --path ./output_path --codec mp3 --bitrate 320 --chunk 1
```

Download a single track by providing its Spotify ID, desired output path, codec (e.g., mp3), bitrate (e.g., 320 kbps), and optionally specifying the number of parallel downloads (--chunk)

### Playlist Download

```sh
cargo run -- --spotify playlist --spotify_id <playlist_id> --path ./output_path --codec mp3 --bitrate 320 --chunk 4
```

Download an entire playlist by specifying its Spotify ID, output path, codec (e.g., mp3), bitrate (e.g., 320 kbps), and optionally specifying the number of parallel downloads (--chunk).\

### Album Download

```sh
cargo run -- --spotify album --spotify_id <album_id> --path ./output_path --codec flac --bitrate best --chunk 2
```

Download a full album by providing its Spotify ID, output path, codec (e.g., flac), best available bitrate, and optionally specifying the number of parallel downloads (--chunk).

| Option                        | Description                                         |
| ----------------------------- | --------------------------------------------------- |
| `--spotify <spotify>`       | Name of the Spotify entity to download.             |
| `--spotify_id <spotify_id>` | ID of the Spotify entity to download.               |
| `--path <file_path>`        | Path where the audio file is to be downloaded.      |
| `--codec <codec>`           | Codec for the downloaded audio file (default: mp3). |
| `--bitrate <bitrate>`       | Bitrate for the downloaded audio file.              |
| `--chunk <chunk>`           | Number of parallel downloads at a time.             |
| `-h, --help`                | Print help                                          |

### Possible Values

| Option      | Possible Values                                   |
| ----------- | ------------------------------------------------- |
| `bitrate` | worst, worse, poor, low, medium, good, high, best |
| `codec`   | mp3, mpa, flac, opus                              |

# Screenshots

<img width="1612" alt="Screenshot 2024-07-07 at 21 41 37" src="https://github.com/sabinonweb/yuck_premium/assets/123313687/750deb98-a839-4763-bb0f-7dbc1d3d7ebd">
 
<img width="845" alt="Screenshot 2024-07-07 at 21 42 07" src="https://github.com/sabinonweb/yuck_premium/assets/123313687/50adbefb-0a96-4173-951d-947fd7ef98ba">

<img width="845" alt="Screenshot 2024-07-07 at 21 44 20" src="https://github.com/sabinonweb/yuck_premium/assets/123313687/57502984-420c-45f2-a443-3e3ce333eb1e">

# Acknowledgements

- spotify-dl : https://github.com/dhruv-ahuja/spoti-dl
