extern crate serde_json;
use serde_json::Value;
use reqwest::{Response};
use dotenv;
use steamgriddb_api::{QueryType::Icon};
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let rpc_client_id = dotenv::var("DISCORD_APPLICATION_ID").expect("Requires at least one argument");
    let api_key = dotenv::var("STEAM_API_KEY").expect("Requires at least one argument");
    let steam_id = dotenv::var("STEAM_USER_ID").expect("Requires at least one argument");
    let griddb_key = dotenv::var("STEAM_GRID_API_KEY").unwrap();

    // Create the client
    let mut drpc = DiscordIpcClient::new(rpc_client_id.as_str()).expect("Failed to create Discord RPC client");

    // Start up the client connection, so that we can actually send and receive stuff
    let mut connected: bool = false;
    let mut start_time: i64 = 0;
    loop {
        let message = get_steam_presence(&api_key, &steam_id).await.unwrap();
        let state_message = message[1..message.len() - 1].to_string();

        let mut img: String = "".to_string();
        if griddb_key != "".to_string() && state_message != "ul" {
            img = steamgriddb(&griddb_key, &state_message).await.unwrap();
        }

        // Set the activity
        if state_message != "ul" {
            if connected != true {
                drpc.connect().expect("Failed to connect to Discord RPC client");
                start_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
            }
            connected = true;
            if img != "".to_string() {
                drpc.set_activity(
                    activity::Activity::new()
                    .state(&state_message.as_str())
                    .timestamps(activity::Timestamps::new()
                        .start(start_time)
                    )
                    .assets(
                        activity::Assets::new()
                            .large_image(img.as_str())
                            .large_text("https://github.com/Radiicall/steam-presence-on-discord") 
                    )
                ).expect("Failed to set activity");
            } else {
                drpc.set_activity(
                    activity::Activity::new()
                    .state(&state_message)
                ).expect("Failed to set activity");
            }
        } else if connected == true {
            drpc.close().expect("Failed to close Discord RPC client");
            let _ = connected == false;
        }
    std::thread::sleep(std::time::Duration::from_secs(18));
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
