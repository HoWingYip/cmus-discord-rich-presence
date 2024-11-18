# Discord Rich Presence integration for cmus

This project integrates the [cmus](https://github.com/cmus/cmus) music player with Discord Rich Presence. It displays the title, artist, and playback progress of the currently playing track. Album art is also retrieved from the [iTunes Search API](https://performance-partners.apple.com/search-api) and displayed if found.


## Installation

First ensure you have Rust installed; if not, [install it](https://www.rust-lang.org/tools/install) using your method of choice. Then run:

```bash
git clone https://github.com/HoWingYip/cmus-discord-rich-presence
cd cmus-discord-rich-presence
./scripts/install.sh
```

Recommended: to automatically start the program on login, additionally run:

```bash
./scripts/add-service.sh
```

The above command will only work if you're using systemd.


## Usage

If you added the service as recommended above, no further action is needed.

If you didn't, you'll need to manually run `cmus-discord-rich-presence` in your shell of choice. Optionally run it in the background using `cmus-discord-rich-presence & disown`.


## Uninstallation

```bash
./scripts/uninstall.sh
```

The above script deletes the binary from `~/.local/bin/` and removes the service if installed.


## Sample screenshot

![Sample screenshot](readme-assets/screenshot.png)
