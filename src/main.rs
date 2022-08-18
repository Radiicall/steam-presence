extern crate serde_json;
use serde_json::Value;
use reqwest::{Response};
use dotenv;
use steamgriddb_api::{QueryType::Icon};
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::io::{Write, BufReader, BufRead};

const PATH: &str = "./.env";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();
    let rpc_client_id = dotenv::var("DISCORD_APPLICATION_ID").unwrap_or_else(|_| "".to_string());
    let api_key = dotenv::var("STEAM_API_KEY").unwrap_or_else(|_| "".to_string());
    let steam_id = dotenv::var("STEAM_USER_ID").unwrap_or_else(|_| "".to_string());
    let griddb_key = dotenv::var("STEAM_GRID_API_KEY").unwrap_or_else(|_| "".to_string());

    if rpc_client_id == "" || api_key == "" || steam_id == "" {
        // Create input
        let mut input = String::new();
        // Ask for discord id
        println!("Please enter your Discord Application ID:");
        // Read line
        std::io::stdin().read_line(&mut input).unwrap();
        // Add line to req
        let mut req = format!("DISCORD_APPLICATION_ID={}\n", input.trim());
        // Reset input to empty
        input = "".to_string();
        // Ask for steam api key
        println!("Please enter your Steam API Key :");
        // Read line
        std::io::stdin().read_line(&mut input).unwrap();
        // Add line to req
        req = format!("{}STEAM_API_KEY={}\n", req, input.trim());
        // Reset input to empty
        input = "".to_string();
        // Ask for steam user id
        println!("Please enter your Steam User ID :");
        // Read line
        std::io::stdin().read_line(&mut input).unwrap();
        // Add line to req
        req = format!("{}STEAM_USER_ID={}\n", req, input.trim());
        // Reset input to empty
        input = "".to_string();
        // Ask for steam grid api key (optional)
        println!("Please enter your Steam Grid API Key (Keep empty if you dont want pictures):");
        // Read line
        std::io::stdin().read_line(&mut input).unwrap();
        // Check if line is empty
        if input.trim() != "" {
            // Add line to req
            req = format!("{}STEAM_GRID_API_KEY={}\n", req, input.trim());
        }
        // Create and write to file
        write(req.as_str()).expect("Failed to write to .env file");
        println!("Please restart the program");
        // Exit program
        std::process::exit(0);
    }

    // Create variables early
    let mut connected: bool = false;
    let mut start_time: i64 = 0;
    let mut drpc = DiscordIpcClient::new(rpc_client_id.as_str()).expect("Failed to create Discord RPC client, discord is down or the Client ID is invalid.");
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
                let idbrok = get_discord_app(&state_message, rpc_client_id.to_lowercase().to_owned()).await.unwrap();
                let appid = idbrok[1..idbrok.len() - 1].to_string();
                println!("App ID: {}", appid);
                println!("Game: {}", state_message);
                println!("Image: {}", img);
                // Create the client
                drpc = DiscordIpcClient::new(appid.as_str()).expect("Failed to create Discord RPC client, discord is down or the Client ID is invalid.");
                // Start up the client connection, so that we can actually send and receive stuff
                drpc.connect().expect("Failed to connect to Discord RPC client, discord is down or the Client ID is invalid.");
                println!("Connected to Discord RPC client");
                // Set the starting time for the timestamp
                start_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
                // Set connected to true so that we don't try to connect again
                connected = true;
            }
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
            std::thread::sleep(std::time::Duration::from_secs(8));
            // Set connected to false so that we dont try to disconnect again
            connected = false;
            println!("Disconnected from Discord RPC client");
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
    let json: Value = serde_json::from_str(&body).expect("Failed to convert to json, is the steam api key and user id correct?");

    // Get the response from the json
    let response: &&Value = &json.get("response").expect("Couldn't find that");
    // Get players from response
    let players: &Value = response.get("players").expect("Couldn't find that");
    // Get the first player from the players
    let mut game_title: &Value = &players[0]["gameextrainfo"];
    // Check if gameextrainfo is null, if so then check if there are more ID's in the response
    if game_title == &Value::Null && players.as_array().unwrap().len() > 1 {
        for i in 1..players.as_array().unwrap().len() {
            game_title = &players[i]["gameextrainfo"];
            if game_title != &Value::Null {
                break;
            }
        }
    }

    // Return the game title
    Ok(game_title.to_string())
}

async fn get_discord_app(query: &str, rpc_client_id: String) -> Result<String, reqwest::Error> {
    // Create the request
    let url = "https://discordapp.com/api/v8/applications/detectable";
    // Get response
    let res: Response = reqwest::get(url).await?;
    
    // Get the body of the response
    let body = res.text().await?;
    
    // Convert to json
    let json: Vec<Value> = serde_json::from_str(&body).unwrap();

    // Get the response from the json
    let mut id: String = format!("+{}+", rpc_client_id);
    for i in 0..json.len() {
        let mut response: Vec<&str> = Vec::new();
        response.push(&json[i].get("name").unwrap().as_str().unwrap());
        if response.contains(&query) {
            id = json[i].get("id").unwrap().to_string();
            break
        }
    }
    Ok(id)
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

 fn write(input: &str) -> Result<(), std::io::Error> {
    // Try to create the file
    let mut output = std::fs::File::create(PATH)?;
    // Try to write input to file
    write!(output, "{}", input)?;
    // Try to set input to file
    let input = std::fs::File::open(PATH)?;
    // Read input to string
    let buffered = BufReader::new(input);

    // Try to print the file contents
    for words in buffered.lines() {
        println!("{}", words?);
    }

    // Return Ok
    Ok(())
}

