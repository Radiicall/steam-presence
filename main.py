# creating rich presences for discord
from pypresence import Presence
from time import sleep

# used to get the game's cover art
# the original library is currently broken at the time of writing this, so i'm using a self made fork
from python_steamgriddb.steamgrid import StyleType, PlatformType, MimeType, ImageType, SteamGridDB

# requesting data from steam's API
import requests

# for errors
from datetime import datetime

# for loading the config file
import json
from os.path import exists


def get_config():
    if exists("config.json"):
        with open("config.json", "r") as f:
            return json.load(f)
    
    if exists("exampleconfig.json"):
        with open("exampleconfig.json", "r") as f:
            return json.load(f)
    
    else:
        print(f"ERROR: [{datetime.now().strftime('%d-%b-%Y %H:%M:%S')}] Config file not found. Please read the readme and create a config file.")
        exit()


config = get_config()
if config["KEY"] == "KEY":
    print(f"ERROR: [{datetime.now().strftime('%d-%b-%Y %H:%M:%S')}] Please set your Steam API key in the config file.")
    exit()

if config["USERID"] == "USERID":
    print(f"ERROR: [{datetime.now().strftime('%d-%b-%Y %H:%M:%S')}] Please set your Steam user ID in the config file.\n(note this is not the same as the URL ID - read the readme for more info")
    exit()

KEY = config["KEY"]
USER = config["USERID"]
APP_ID = config["DISCORD_APPLICATION_ID"]

GRID_ENABLED = config["COVER_ART"]["ENABLED"]
GRID_KEY = config["COVER_ART"]["STEAM_GRID_API_KEY"]


def get_steam_presence():
    r = requests.get(f"https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/?key={KEY}&format=json&steamids={str(USER)}").json()

    if len(r["response"]["players"]) == 0:
        print(f"ERROR: [{datetime.now().strftime('%d-%b-%Y %H:%M:%S')}] Player found, this is likely because the userID is invalid, the key is invalid, or steam is down. Please try again later or read thru the readme again.")

    try:
        game_title = None
        for i in r["response"]["players"][0]:
            if i == "gameextrainfo":
                game_title = r["response"]["players"][0][i]

        if game_title is not None:
            return game_title

    except:
        pass

def get_steam_grid_icon(gameName):
    with open(f'icons.txt', 'r') as icons:
        for i in icons:
            if gameName in i:
                return i.split("=")[1]
        
    results = sgdb.search_game(gameName)

    # yes this is terrible code but i really couldn't figure out a better way to do this, sorry - pull request anyone?
    result = str(results).split(',')[0][1:]
    steamGridAppID = result[9:].split(' ')[0]
    
    resolutions = [
        512,
        256,
        128,
        64,
        32,
        16
    ]
    
    grids = sgdb.get_icons_by_gameid(game_ids=[steamGridAppID])
    icon = grids[0]
    
    # basically some of the icons are .ico files, discord cannot display these
    # what this does is basically brute force test a bunch of resolutions and pick the first one that works
    # as steamgriddb actually hosts png versions of all the .ico files, they're just not returned by the API
    for res in resolutions:
        icon = str(icon)
        newURL = icon[:-4] + f"/32/{res}x{res}.png"
        
        r = requests.get(newURL)
        if r.status_code == 200:
            break

        if res == 16:
            return None
        
    
    with open(f'icons.txt', 'a') as icons:
        icons.write(f"{gameName}={newURL}\n")
        icons.close()
    return newURL



if __name__ == "__main__":
    if GRID_ENABLED:
        sgdb = SteamGridDB(GRID_KEY)
    
    RPC = Presence(client_id=APP_ID)
    RPC.connect()
    
    while True:
        game_title = get_steam_presence()
        #print(game_title)
        
        if game_title is None:
            # note, this completely hides your current rich presence
            RPC.clear()
            
        else:
            if GRID_ENABLED:
                coverImage = get_steam_grid_icon(game_title)
                RPC.update(state=game_title, large_image=f"{coverImage[:-1]}", large_text=f"{game_title}")
            else:
                RPC.update(state=game_title)
            
        sleep(20)
