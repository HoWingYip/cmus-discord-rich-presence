set -e

cargo build --release
cp target/release/cmus-discord-rich-presence $HOME/.local/bin

echo "Installed to $HOME/.local/bin/cmus-discord-rich-presence"
