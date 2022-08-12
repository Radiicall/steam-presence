extern crate serde_json;
use serde_json::Value;
use reqwest::{Response};
use std::env;
use steamgriddb_api::{QueryType::Icon};

#[tokio::main]
async fn main() {
    let rpc_client_id = env::args().nth(1).expect("Requires at least one argument");
    let api_key = env::args().nth(2).expect("Requires at least one argument");
    let steam_id = env::args().nth(3).expect("Requires at least one argument");
    let griddb_key = env::args().nth(4).unwrap();

    // Create the client
    let mut drpc = discord_presence::Client::new(rpc_client_id.parse::<u64>().unwrap());

    // Register event handlers with the corresponding methods
    drpc.on_ready(|_ctx| {
        println!("Ready!");
    });

    // Start up the client connection, so that we can actually send and receive stuff
    drpc.start();

    loop {
        let message = get_steam_presence(&api_key, &steam_id).await.unwrap();
        let state_message = message[1..message.len() - 1].to_string();

        let mut img: String = "".to_string();
        if griddb_key != "".to_string() && state_message != "ul" {
            img = steamgriddb(&griddb_key, &state_message).await.unwrap();
        }

        // Set the activity
        if state_message != "ul" {
            if img != "".to_string() {
                if let Err(why) = drpc.set_activity(|a| {
                    a.state(state_message)
                    .timestamps(|ts|
                        ts.start(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as u64)
                    )
                    .assets(|ass| {
                        ass.large_image(img)
                            .large_text("https://github.com/Radiicall/steam-presence-on-discord") 
                    })
                }) {
                    println!("Failed to set presence: {}", why);
                }
            } else {
                if let Err(why) = drpc.set_activity(|a| {
                    a.state(state_message)
                }) {
                    println!("Failed to set presence: {}", why);
                }
            }
        } else {
            if let Err(why) = drpc.set_activity(|a| {
                a.state("")
            }) {
                println!("Failed to set presence: {}", why);
            }
        }
    std::thread::sleep(std::time::Duration::from_secs(20));
    }
}

async fn get_steam_presence(api_key: &String, steam_id: &String) -> Result<String, reqwest::Error> {
    /* You don't need to actually write the return types.
     * Rust will do that for you */
     let url = format!("https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/?key={}&format=json&steamids={}", api_key, steam_id);
     let res: Response = reqwest::get(url).await?;

     let body = res.text().await?;

     // NOTE: This is important to be able to use your Json like Python/Javascript
     let json: Value = serde_json::from_str(&body).unwrap();

     // Getting values from Json is like getting values from a Hashmap.
     let response: &&Value   = &json.get("response").expect("Couldn't find that");
     let players: &Value = response.get("players").expect("Couldn't find that");
     let game_title: &Value = &players[0]["gameextrainfo"];

     Ok(game_title.to_string())
}

async fn steamgriddb(griddb_key: &String, query: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = steamgriddb_api::Client::new(griddb_key);
    let games = client.search(query).await?;
    let first_game = games.iter().next();
    let mut image: String = "".to_string();
    if let Some(first_game) = first_game {
        let images = client.get_images_for_id(first_game.id, &Icon(None)).await?;
        image = images[0].url.to_string()
    }
    Ok(image)
 }
