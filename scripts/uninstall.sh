case $(systemctl --user list-units --type=service --all) in
  *cmus-discord-rich-presence.service*)
    echo "Found service; uninstalling"
    
    systemctl --user disable --now cmus-discord-rich-presence.service
    rm $HOME/.local/share/systemd/user/cmus-discord-rich-presence.service
    
    echo "Uninstalled service."
    ;;
esac

BINARY_PATH=$HOME/.local/bin/cmus-discord-rich-presence
rm $BINARY_PATH
echo "Removed '$BINARY_PATH'"
