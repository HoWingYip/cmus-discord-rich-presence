set -e

EXECUTABLE_PATH=$HOME/.local/bin/cmus-discord-rich-presence

if [ ! -f $EXECUTABLE_PATH ]; then
  echo "Executable not found at $EXECUTABLE_PATH; exiting"
  exit
fi

USER_SERVICE_DIR=$HOME/.local/share/systemd/user

mkdir -p $USER_SERVICE_DIR

cat >$USER_SERVICE_DIR/cmus-discord-rich-presence.service <<EOL
[Unit]
Description=Discord Rich Presence integration for cmus

[Service]
Type=simple
ExecStart=$EXECUTABLE_PATH

[Install]
WantedBy=default.target
EOL
echo "Installed user service."

systemctl --user enable --now cmus-discord-rich-presence.service
echo "Enabled and started user service."

echo "cmus-discord-rich-presence will now run on login for the current user."
