### Table of Contents
- [Steam Presence on Discord](#steam-presence-on-discord)
  - [Why](#why)
- [Setup](#setup)
  - [Discord Application ID](#discord-application-id)
  - [Steam Web API](#steam-web-api)
  - [Steam User ID](#steam-user-id)
  - [Cover Art](#cover-art)
    - [SteamGridDB](#steamgriddb)
    - [Custom](#custom)
  - [Systemd](#systemd)

<p align='center'>
  <img src="readmeimages/banner.png" alt="Steam Presence on Discord logo">
</p>

# Steam Presence on Discord

A simple script to check a Steam user's current game, and display that as a Discord rich presence.

![ExampleImage1](readmeimages/example1.png)

Playing "BTD6" with the script running 

![ExampleImage2](readmeimages/example2.png)

Playing "Everything" with the script running (more niche game so Discord doesn't have it saved)

### Why??
Well, why did i make this? Discord already detects the games you're playing so isn't this just pointless??

See, no.

Discord has severe limitations when it comes to Linux as most games running through a compatability layer (like 90% of them) are displayed as pr-wrap or something similar. Also the flatpak version of Discord has no support for detecting games at all while still having Rich Presence, so this is the only way to show "playing" status.

In addition to this, there's the Steam Deck, a handheld linux game "console".

Having discord constantly run in the background is a terrible idea considering how that's gonna lose you at least half an hour of battery life, in addition to the previous issues with Linux.

So this script is a way of circumventing these issues by instead having this run on something like a server 24/7.

Also yes this is very dumb you're right lmao


# Setup
Run the executable to get an interactive way to create the config

**or**

Create a file named `.env` in the top directory and fill it.
 
```
DISCORD_APPLICATION_ID=

STEAM_API_KEY=

STEAM_USER_ID=

RETRY_COUNT=3

STEAM_GRID_API_KEY=
```


## Discord Application ID
The `DISCORD_APPLICATION_ID` is the discord application ID of the app you want to use.

Please generate one [here](https://discordapp.com/developers/applications/) or use Temmie's "869994714093465680"

The only thing you need to fill out on their site is the application name itself.

For example Temmie's is called "a game on steam" as shown in the screenshot above.


## Steam Web API
The `STEAM_API_KEY` in this case is regarding to the Steam web API.

This you can obtain by registering [here](https://steamcommunity.com/dev/apikey) while logged in

## Steam User ID
The `STEAM_USER_ID` is the steam user id of the user you want to track.

**NOTE** This is not the same as the display URL of the user.

The easiest way i've found to get the ID is by throwing your url into the [SteamDB Calculator](https://steamdb.info/calculator/)

and then taking the ID from that url

![ExampleImage](readmeimages/steamDB.png)


## Cover Art
And then we have the Cover Art section.

Having any cover art at all is optional. You can completely ignore this section without any errors.

**NOTE** If both SteamGridDB and Custom are used it will choose the Custom icon over SteamGridDB

### SteamGridDB
This will use an icon from SteamGridDB as the cover art for the discord presence.

You can get your API key [here](https://www.steamgriddb.com/profile/preferences/api)

### Custom
Create a file named icons.txt in the same folder as the executable

Add your game name to it and a URL to the image you want to use

Example:
```
Deep Rock Galactic=https://cdn2.steamgriddb.com/file/sgdb-cdn/icon/fb508ef074ee78a0e58c68be06d8a2eb/32/256x256.png
Apex Legends=https://cdn.discordapp.com/attachments/1008823510992433226/1010193491483164784/21509-256x256x32.png
Trailmakers=https://s1.qwant.com/thumbr/0x0/0/e/042f3e3c97b657ad274223498150c95d35516190b32647708cdd37cd3de767/trailmakers-logo.png?u=https%3A%2F%2Fupmychrome.com%2Fimages%2Fuploads%2Fproducts%2F1912%2Ftrailmakers-logo.png
```


## Systemd
To use the systemd service run the `install-service.sh` file without root after filling your .env file

**NOTE:** If you can't run the file do `chmod +x install-service.sh`
