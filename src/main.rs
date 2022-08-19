extern crate serde_json;
use serde_json::Value;
use reqwest::{Response};
use dotenv;
use steamgriddb_api::{QueryType::Icon};
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::io::{Write, BufReader, BufRead};

const ENV: &str = "./.env";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();
    let rpc_client_id = dotenv::var("DISCORD_APPLICATION_ID").unwrap_or_else(|_| "".to_string());
    let api_key = dotenv::var("STEAM_API_KEY").unwrap_or_else(|_| "".to_string());
    let steam_id = dotenv::var("STEAM_USER_ID").unwrap_or_else(|_| "".to_string());
    let retrycount = dotenv::var("RETRY_COUNT").unwrap_or_else(|_| "3".to_string()).parse::<u64>().expect("RETRY_COUNT must be a number");
    let griddb_key = dotenv::var("STEAM_GRID_API_KEY").unwrap_or_else(|_| "".to_string());
    println!("//////////////////////////////////////////////////////////////////\nSteam Presence on Discord\nhttps://github.com/Radiicall/steam-presence-on-discord");
    if rpc_client_id == "" || api_key == "" || steam_id == "" {
        // Run setup
        setup_env();
        std::process::exit(0);
    }

    // Steam ID(s)
    println!("//////////////////////////////////////////////////////////////////\nSteam ID(s):\n{}", steam_id.replace(",", "\n"));

    // Create variables early
    let mut connected: bool = false;
    let mut start_time: i64 = 0;
    let mut drpc = DiscordIpcClient::new(rpc_client_id.as_str()).expect("Failed to create Discord RPC client, discord is down or the Client ID is invalid.");
    let mut img: String = "".to_string();
    let mut curr_state_message: String = "".to_string();
    // Start loop
    loop {
        // Get the current open game in steam
        let message = get_steam_presence(&api_key, &steam_id, retrycount).await.unwrap();
        let state_message = message[1..message.len() - 1].to_string();

        if state_message != "ul" {
            if connected != true {
                // Grab image from griddb if it is enabled
                if griddb_key != "".to_string() && state_message != "ul" {
                    // Get image from griddb
                    img = steamgriddb(&griddb_key, &state_message).await.unwrap();
                }
                // Read icons.txt
                let icons = read_icons().unwrap_or_else(|_| "".to_string());
                if icons != "" && state_message != "ul" {
                    // Find icon in icons
                    let icon = icons.split("\n").find(|icon| icon.contains(&state_message)).unwrap_or_else(|| "");
                    // Check if icon is empty
                    if icon != "" {
                        // Set img to icon
                        img = icon.split("=").nth(1).unwrap().to_string();
                    }
                }
                let idbrok = get_discord_app(&state_message, rpc_client_id.to_owned()).await.unwrap();
                let appid = idbrok[1..idbrok.len() - 1].to_string();
                println!("//////////////////////////////////////////////////////////////////\nApp ID: {}\nGame: {}\nImage: {}", appid, state_message, img);
                // Create the client
                drpc = DiscordIpcClient::new(appid.as_str()).expect("Failed to create Discord RPC client, discord is down or the Client ID is invalid.");
                // Start up the client connection, so that we can actually send and receive stuff
                drpc.connect().expect("Failed to connect to Discord RPC client, discord is down or the Client ID is invalid.");
                println!("//////////////////////////////////////////////////////////////////\nConnected to Discord RPC client");
                // Set the starting time for the timestamp
                start_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
                // Set current state message
                curr_state_message = state_message.to_string();
                // Set connected to true so that we don't try to connect again
                connected = true;
            } else if state_message != curr_state_message {
                 // Disconnect from the client
                drpc.close().expect("Failed to close Discord RPC client");
                std::thread::sleep(std::time::Duration::from_secs(8));
                // Set connected to false so that we dont try to disconnect again
                connected = false;
                println!("Disconnected from Discord RPC client");
                std::thread::sleep(std::time::Duration::from_secs(18));
                continue;
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

async fn get_steam_presence(api_key: &String, steam_id: &String, retrycount: u64) -> Result<String, reqwest::Error> {
    // Convert to json
    let mut body: String = "".to_string();
    for i in 1..retrycount {
        if i > 1 {
            println!("Failed to connect to steam, retrying...");
            std::thread::sleep(std::time::Duration::from_secs(10));
        }

        // Create the request
        let url = format!("https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/?key={}&format=json&steamids={}", api_key, steam_id);
        // Get response
        let res: Response = reqwest::get(url).await?;

        if res.status() != 200{
            continue;
        }

        // Get the body of the response
        body = res.text().await?;
        break;
    }
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
        if images.len() > 0 {
            image = images[0].url.to_string();
            if !image.ends_with(".png") {
                let resolutions = vec![
                    512,
                    256,
                    128,
                    64,
                    32,
                    16
                ];
                for res in resolutions {
                    let url = format!("{}/32/{}x{}.png", image[0..image.len() - 4].to_string(), res, res);

                    let r: Response = reqwest::get(url.as_str()).await?;

                    if r.status() == 200 {
                        image = url;
                        break;
                    }
                }
            }
        }
    }
    Ok(image)
 }

 fn setup_env() {
    // Create input
    let mut input = String::new();
    // Ask for discord id
    println!("//////////////////////////////////////////////////////////////////\nPlease enter your Discord Application ID:");
    // Read line
    std::io::stdin().read_line(&mut input).unwrap();
    // Add line to req
    let mut req = format!("DISCORD_APPLICATION_ID={}\n", input.trim());
    // Reset input to empty
    input = "".to_string();
    // Ask for steam api key
    println!("\nPlease enter your Steam API Key:");
    // Read line
    std::io::stdin().read_line(&mut input).unwrap();
    // Add line to req
    req = format!("{}STEAM_API_KEY={}\n", req, input.trim());
    // Reset input to empty
    input = "".to_string();
    // Ask for steam user id(s)
    println!("\nPlease enter your Steam User ID(s) (Seperated by commas '12345,67890'):");
    // Read line
    std::io::stdin().read_line(&mut input).unwrap();
    // Add line to req
    req = format!("{}STEAM_USER_ID={}\n", req, input.trim());
    // Reset input to empty
    input = "".to_string();
    // Ask for steam user id(s)
    println!("\nPlease enter how many times to retry steam API connection (Default: 3):");
    // Read line
    std::io::stdin().read_line(&mut input).unwrap();
    // Add line to req
    if input.trim() != "" {
        req = format!("{}RETRY_COUNT={}\n", req, input.trim());
    }
    // Reset input to empty
    input = "".to_string();
    // Ask for steam grid api key (optional)
    println!("\nPlease enter your Steam Grid API Key (Keep empty if you dont want/need pictures):");
    // Read line
    std::io::stdin().read_line(&mut input).unwrap();
    // Add line to req
    if input.trim() != "" {
        req = format!("{}STEAM_GRID_API_KEY={}\n", req, input.trim());
    }
    // Create and write to file
    write(req.as_str()).expect("Failed to write to .env file");
    println!("//////////////////////////////////////////////////////////////////\nPlease restart the program");

 }

 fn write(input: &str) -> Result<(), std::io::Error> {
    // Try to create the file
    let mut output = std::fs::File::create(ENV)?;
    // Try to write input to file
    write!(output, "{}", input)?;
    // Try to set input to file
    let input = std::fs::File::open(ENV)?;
    // Read input to string
    let buffered = BufReader::new(input);

    // Try to print the file contents
    for words in buffered.lines() {
        println!("{}", words?);
    }

    // Return Ok
    Ok(())
}

fn read_icons() -> Result<String, std::io::Error>{
    let icons = std::env::current_exe()?.parent().unwrap().join("icons.txt");
    // Open file and read to string
    return Ok(std::fs::read_to_string(icons)?);
}
