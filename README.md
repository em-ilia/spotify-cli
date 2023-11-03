# spotify-cli (i am legally obligated to pick bland titles)

I will lose my mind if my Taylor Mix playlist is not sorted by album at all times.

## Features
- Easy (relative) **auth**entication with a spotify developer account
- Playlist functions
  - **Copy** contents of a playlist into a different playlist
  - **Sort** contents of a playlist by multiple factors
    - Album release
    - Track number (order on album)
  - **Clear** a playlist
  - Create a **new** playlist
  - Dump playlist contents (for backup)

## Usage
Pass `-h` or `--help` to see mostly helpful usage notes (thanks [clap](https://github.com/clap-rs/clap)).

### Setup and Authentication
Do note that you'll need a Spotify account and an "app" ([make one here](https://developer.spotify.com/dashboard)).
You'll copy out the `Client ID` and `Client Secret`,
and put them in `~/.config/spotify-cli` or some other directory of your choosing in a file called `config`.
The format is
```
client_id = "The thing you copied"
client_secret = "The other thing you copied"
```
Then run `./spotify-cli auth` and follow the given instructions.

One note:
- `copy A into B` means we are altering playlist `B` and reading from playlist `A`
- `copy A from B` means we are altering playlist `A` and reading from playlist `B`

In retrospect, I could have been more prescriptive here 🤷‍♀️

## Installation
1. Acquire a rust toolchain
   - You should probably use `rustup`. Get that [here](https://rustup.rs/).
2. Clone this repo
3. Run `cargo build -r` in the clone directory

cargo my beloved

## Example
Here I assume you've already followed the steps in [Usage](#Usage).

Get yourself a nice clean playlist by running `./spotify-cli playlist new "NAME-OF-PLAYLIST"`

<img src="https://github.com/em-ilia/spotify-cli/assets/23224059/21d4ba08-089d-4abe-a43f-16b7b66f294b" width=60%>

If you look at the URL for that playlist (either share it or *use the browser client*),
you'll get a URL like `https://open.spotify.com/playlist/cool-funky-letters-and-numbers`.
The last part is the playlist URI, which you'll want to copy.
If you already have a playlist you want to paste into this new one, get its URI too.
Then if we run `./spotify-cli playlist copy SOURCE-URI into DEST-URI` we'll populate our new playlist.

<img src="https://github.com/em-ilia/spotify-cli/assets/23224059/3facb601-61c1-439b-a40a-74c784a210e0" width=60%>

That's great, but isn't it disgusting how we have songs from *rep* mixed in with songs from *Lover*?
Even worse, we have End Game after Delicate, even though End Game appears first on the album.
To fix this, we run `./spotify-cli playlist sort SOURCE-URI album-release track-number`.

<img src="https://github.com/em-ilia/spotify-cli/assets/23224059/b0ab6f03-cdff-4300-bcf3-b502c61743b2" width=60%>

Now everything is as it should be. The songs from *reputation* appear first because chronologically,
it was the first album to be released (Taylor's Versions count as being later 😮‍💨 and I don't like self-titled).

Alternatively, if you run `./spotify-cli playlist sort SOURCE-URI track-number album-release`
you'll get a playlist sorted first by track number, then by album release.
Personally, I hate this. It makes me uncomfortable. I find it unseemly.

<img src="https://github.com/em-ilia/spotify-cli/assets/23224059/b5a18484-890a-4211-84be-cb6dbc8acd64" width=60%>


