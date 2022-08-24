#!/bin/bash

if [ `whoami` == "root" ];
then
    echo "This script cannot be ran as root"
    exit 1
fi

sed "s/CHANGEME/$USER/g" steam-presence.service > steam-presence.service.tmp

# Find .env file
if [ -f .env ];
then
    echo "Found .env file"
else
    echo "No .env file found"
    echo "Please create one or run the 'steam-presence-on-discord' executable to make one automatically"
    exit 1
fi

systemctl --user stop steam-presence.service

echo "Installing systemd service"
cp steam-presence.service.tmp ~/.config/systemd/user/steam-presence.service
rm steam-presence.service.tmp

echo "Moving files to ~/.config/steam-presence"
mkdir -p ~/.config/steam-presence
cp steam-presence-on-discord ~/.config/steam-presence/steam-presence-on-discord
cp .env ~/.config/steam-presence/.env

echo "Enable and start the service with 'systemctl --user enable steam-presence.service && systemctl --user start steam-presence.service'"
exit 0
