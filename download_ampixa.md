# Downloading songs for Ampixa

How to go from "I have a Spotify account" to a folder of tagged `.mp3` files with lyrics, using `yuck_premium`.

Time needed: about 30 minutes of setup, then the download runs on its own (about 4–6 songs per minute).

---

## 1. Install the tools (once)

You need Rust, FFmpeg and yt-dlp.

```sh
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# FFmpeg (converts the audio)
brew install ffmpeg

# yt-dlp (fetches the audio). If `brew install yt-dlp` fails, use pip:
python3 -m pip install -U yt-dlp
```

Check that everything is installed:

```sh
cargo --version && ffmpeg -version | head -1 && yt-dlp --version
```

## 2. Get the code

```sh
git clone https://github.com/sabinonweb/yuck_premium
cd yuck_premium
cargo build
```

## 3. Create your own Spotify app (once)

Everyone needs their **own** app. Don't share client secrets.

1. Go to <https://developer.spotify.com/dashboard> and log in.
2. Click **Create app**.
   - **Name / description:** anything, e.g. `yuck-<yourname>`
   - **Redirect URI:** `http://127.0.0.1:6375/callback`. It must be exactly this, since Spotify no longer accepts `localhost`.
   - **APIs used:** tick **Web API**
3. Save, open **Settings**, and copy the **Client ID** and **Client secret**.

## 4. Create `.env` (once)

In the `yuck_premium` folder, create a file named `.env`:

```
RSPOTIFY_CLIENT_ID=<your client id>
RSPOTIFY_CLIENT_SECRET=<your client secret>
RSPOTIFY_REDIRECT_URI=http://127.0.0.1:6375/callback
YUCK_DOWNLOAD_DIR=/absolute/path/where/songs/should/go
```

`.env` is in `.gitignore`. **Never commit it.**

## 5. Make a playlist you own

Spotify only lets development apps read the songs of playlists **you own**. Someone else's playlist (even a public one) fails with `missing items/tracks`.

To use someone else's playlist, copy it into your account:

1. Open the playlist in the Spotify desktop app.
2. Click any song, press **Cmd+A** (Ctrl+A on Windows) to select all.
3. Right-click, then choose **Add to playlist → New playlist**.

Or build your own playlist by hand. Either way, get its ID:

- Playlist, then **Share → Copy link to playlist**
- The link looks like `https://open.spotify.com/playlist/7fGZgu0vH4iCT3eXzAc9ld?si=...`
- The ID is the part after `/playlist/` and before `?`, here `7fGZgu0vH4iCT3eXzAc9ld`

## 6. First run: log in to Spotify

The first run opens your browser to approve the app.

1. Run the download command (step 7).
2. Approve in the browser. You'll land on a page that fails to load. **That's expected.**
3. Copy the full URL from the browser's address bar (it starts with `http://127.0.0.1:6375/callback?code=...`).
4. Paste it into the terminal where it asks `Please enter the URL you were redirected to:` and press Enter.

The login is cached in `.spotify_token_cache.json`, so later runs skip this step. That file is also secret; never commit it.

## 7. Download

```sh
cargo run --release -- --spotify playlist --spotify_id <PLAYLIST_ID> --codec mp3 --bitrate best --chunk <N>
```

**Choosing `--chunk`:** it's the number of songs **per group**, and each group downloads in parallel. So:

```
parallel downloads = number of songs ÷ chunk
```

Aim for about **4 parallel downloads**. More than that risks YouTube blocking you.

| Playlist size | `--chunk` |
|---|---|
| 100 songs | `25` |
| 1,000 songs | `250` |
| 2,700 songs | `700` |

Other options:
- `--spotify track` or `--spotify album` with a track or album ID also works.
- `--codec`: `mp3`, `flac`, `opus` or `mpa`
- `--bitrate`: `worst`, `worse`, `poor`, `low`, `medium`, `good`, `high` or `best`
- `--path <dir>` overrides `YUCK_DOWNLOAD_DIR` for one run.

**Long runs:** keep the laptop awake and save a log:

```sh
caffeinate -i cargo run --release -- --spotify playlist --spotify_id <ID> --codec mp3 --bitrate best --chunk 700 > run.log 2>&1 &
```

Check progress with:

```sh
grep -c "downloaded!" run.log
```

## 8. What you get

```
$YUCK_DOWNLOAD_DIR/
└── <Playlist name>/
    ├── Song Name.mp3      ← audio, tagged: title, artist, album, track no., cover art, lyrics (if found)
    ├── Song Name.lrc      ← time-synced lyrics (only if found)
    └── SongName.jpeg      ← album cover
```

Lyrics come from [LRCLIB](https://lrclib.net), a free, crowd-sourced database. Songs without lyrics still download, and the program prints a yellow `WARN: No lyrics found for ...`.

---

## Known issues (read before using the data)

- **Lyrics coverage is low for Nepali songs.** Expect many songs with no lyrics.
- **Some lyrics are junk.** LRCLIB is crowd-sourced; at least one entry contained just the word `probe`. Nothing filters these yet, so spot-check them.
- **The YouTube match is the first search result.** Occasionally it's a live version, a cover or the wrong song. Spot-check them.
- **Failed downloads don't stop the run.** Look for red `Error while downloading` lines in the log.
- **The exit code is always 0**, even when songs fail.
- **Songs with the same title overwrite each other** inside one playlist folder.

## Troubleshooting

| Error | Cause / fix |
|---|---|
| `RSPOTIFY_CLIENT_ID ... NotPresent` panic | `.env` is missing or in the wrong folder. Run from inside `yuck_premium/`. |
| `missing items/tracks` | You don't own the playlist. Copy it first (step 5). |
| `INVALID_CLIENT: Invalid redirect URI` | The redirect URI in the dashboard and `.env` don't match exactly. |
| `io error: No such file or directory` on every song | `yt-dlp` isn't installed or isn't on `PATH`. |
| `brew install yt-dlp` lock error | Use `python3 -m pip install -U yt-dlp` instead. |

## Rights

Before songs from this go into an Ampixa dataset, confirm with the team that we have the rights to use them. Spotify's and YouTube's terms restrict downloading and ML use, and the songs and lyrics are copyrighted.
