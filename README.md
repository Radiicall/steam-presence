# Heads Up
This code and Temmie's code are fundamentally different.
They do the same thing in different ways and have different features.

# steam presence on discord

a simple script to check a Steam user's current game, and display that as a Discord rich presence

![ExampleImage1](readmeimages/example1.png)

playing "BTD6" with the script running 

![ExampleImage2](readmeimages/example2.png)

playing "Everything" with the script running (more niece game so Discord doesn't have it saved)

### Why??
well, why did i make this? Discord already detects the games you're playing so isn't this just pointless??

see, no.

Discord has severe limitations when it comes to Linux as most games running thru a compatability layer (like 90% of them) are displayed as pr-wrap or something similar.

in addition to this, there's the Steam Deck, a handheld linux game "console".

having discord constantly run in the background is a terrible idea considering how that's gonna lose you at least half an hour of battery life, in addition to the previous issues with linux.

so this script is a way of circumventing these issues by instead having this run on something like a server 24/7.

also yes this is very dumb you're right lmao

# Setup
Run the executable to get an interactive way to create the config

**or**

create a file named `.env` in the top directory and fill it.
 
```
DISCORD_APPLICATION_ID=

STEAM_API_KEY=

STEAM_USER_ID=

RETRY_COUNT=3

STEAM_GRID_API_KEY=
```
## Steam web API
the `KEY` in this case is regarding to the Steam web API.

this you can obtain by registering here https://steamcommunity.com/dev/apikey while logged in

## Steam User ID
the `USERID` is the steam user id of the user you want to track.

**NOTE** this is not the same as the display URL of the user.

the easiest way i've found to get the ID is by throwing your url into the steamDB calculator https://steamdb.info/calculator/

and then taking the ID from that url

![ExampleImage](readmeimages/steamDB.png)

## Discord Application ID
the `DISCORD_APPLICATION_ID` is the discord application ID of the app you want to use.

please generate one here https://discordapp.com/developers/applications/ or use Temmie's "869994714093465680"

the only thing you need to fill out on their site is the application name itself.

for example i named mine "a game on steam" as shown in the screenshot above.

## Cover Art (SteamGridDB)
and then we have the `COVER_ART` section.

This will use an icon from steamGridDB as the cover art for the discord presence.

**NOTE** this is optional and the script functions perfectly without it, you'll just be missing the cover art.
To disable this just remove the `STEAM_GRID_API_KEY=` line.

you can get your API key here https://www.steamgriddb.com/profile/preferences/api

## Cover Art (Custom)
Create a file named icons.txt in the same folder as the executable

Add your game name to it and a URL to the image you want to use

Example:
```
Deep Rock Galactic=https://cdn2.steamgriddb.com/file/sgdb-cdn/icon/fb508ef074ee78a0e58c68be06d8a2eb/32/256x256.png
Apex Legends=https://cdn.discordapp.com/attachments/1008823510992433226/1010193491483164784/21509-256x256x32.png
Trailmakers=https://s1.qwant.com/thumbr/0x0/0/e/042f3e3c97b657ad274223498150c95d35516190b32647708cdd37cd3de767/trailmakers-logo.png?u=https%3A%2F%2Fupmychrome.com%2Fimages%2Fuploads%2Fproducts%2F1912%2Ftrailmakers-logo.png
```


