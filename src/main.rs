extern crate serde_json;
use serde_json::Value;
use reqwest::{Response};
use dotenv;
use steamgriddb_api::{QueryType::Icon};
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();
    let rpc_client_id = dotenv::var("DISCORD_APPLICATION_ID").unwrap();
    let api_key = dotenv::var("STEAM_API_KEY").unwrap();
    let steam_id = dotenv::var("STEAM_USER_ID").unwrap();
    let griddb_key = dotenv::var("STEAM_GRID_API_KEY").unwrap();

    if rpc_client_id == "" || api_key == "" || steam_id == "" || griddb_key == "" {
        println!("Please fill in the .env file");
        std::process::exit(1);
    }

    // Create the client
    let mut drpc = DiscordIpcClient::new(rpc_client_id.as_str()).expect("Failed to create Discord RPC client");

    // Create variables early
    let mut connected: bool = false;
    let mut start_time: i64 = 0;
    // Start loop
    loop {
        // Get the current open game in steam
        let message = get_steam_presence(&api_key, &steam_id).await.unwrap();
        let state_message = message[1..message.len() - 1].to_string();

        // Get image from steamgriddb if griddb_key is present
        let mut img: String = "".to_string();
        if griddb_key != "".to_string() && state_message != "ul" {
            img = steamgriddb(&griddb_key, &state_message).await.unwrap();
        }

        
        if state_message != "ul" {
            if connected != true {
                // Start up the client connection, so that we can actually send and receive stuff
                drpc.connect().expect("Failed to connect to Discord RPC client");
                // Set the starting time for the timestamp
                start_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
            }
            // Set connected to true so that we don't try to connect again
            connected = true;
            // Set the activity
            if img != "".to_string() {
                drpc.set_activity(
                    activity::Activity::new()
                    // Set the "state" or message
                    .state(&state_message.as_str())
                    // Add a timestamp
                    .timestamps(activity::Timestamps::new()
                        .start(start_time)
                    )
                    // Add image and a link to the github repo
                    .assets(
                        activity::Assets::new()
                            .large_image(img.as_str())
                            .large_text("https://github.com/Radiicall/steam-presence-on-discord") 
                    )
                ).expect("Failed to set activity");
            } else {
                drpc.set_activity(
                    activity::Activity::new()
                    // Set the "state" or message
                    .state(&state_message)
                ).expect("Failed to set activity");
            }
        } else if connected == true {
            // Disconnect from the client
            drpc.close().expect("Failed to close Discord RPC client");
            // Set connected to false so that we dont try to disconnect again
            let _ = connected == false;
        }
    // Sleep for 18 seconds
    std::thread::sleep(std::time::Duration::from_secs(18));
    }
}

async fn get_steam_presence(api_key: &String, steam_id: &String) -> Result<String, reqwest::Error> {
    // Create the request
    let url = format!("https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/?key={}&format=json&steamids={}", api_key, steam_id);
    // Get response
    let res: Response = reqwest::get(url).await?;

    // Get the body of the response
    let body = res.text().await?;

    // Convert to json
    let json: Value = serde_json::from_str(&body).unwrap();

    // Get the response from the json
    let response: &&Value   = &json.get("response").expect("Couldn't find that");
    // Get players from response
    let players: &Value = response.get("players").expect("Couldn't find that");
    // Get the first player from the players
    let game_title: &Value = &players[0]["gameextrainfo"];

    // Return the game title
    Ok(game_title.to_string())
}

async fn steamgriddb(griddb_key: &String, query: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Create the client
    let client = steamgriddb_api::Client::new(griddb_key);
    // Search for the currently open game
    let games = client.search(query).await?;
    // Get the first game
    let first_game = games.iter().next();
    // Create image variable early so rust doesnt freak out
    let mut image: String = "".to_string();
    // If there is a first game
    if let Some(first_game) = first_game {
        // Get the images of the game
        let images = client.get_images_for_id(first_game.id, &Icon(None)).await?;
        // Get the first image
        image = images[0].url.to_string()
    }
    Ok(image)
 }
