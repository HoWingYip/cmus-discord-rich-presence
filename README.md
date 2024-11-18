# Discord Rich Presence integration for cmus

This project integrates the [cmus](https://github.com/cmus/cmus) music player with Discord Rich Presence. It displays the track title, artist, and playback progress. Album art is also retrieved from the [iTunes Search API](https://performance-partners.apple.com/search-api) and displayed if found.


## Installation

First ensure you have Rust installed. If not, [install it](https://www.rust-lang.org/tools/install) using your method of choice. Then run:

```bash
git clone https://github.com/HoWingYip/cmus-discord-rich-presence
cd cmus-discord-rich-presence
cargo build --release
cp target/release/cmus-discord-rich-presence /usr/local/bin
```

If you want to start `cmus-discord-rich-presence` every time you launch `cmus`, consider adding the following alias to your `~/.bashrc`:

```bash
alias cmus='cmus-discord-rich-presence &>/dev/null & disown && cmus'
```


## Sample screenshot

![Sample screenshot](readme-assets/screenshot.png)
