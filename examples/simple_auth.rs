use std::env;

use soniq::client::Client;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let api_key = env::var("LASTFM_API_KEY").expect("Set LASTFM_API_KEY env var");
    let api_secret = env::var("LASTFM_API_SECRET").expect("Set LASTFM_API_SECRET env var");

    let client = Client::builder(api_key.clone())
        .api_secret(api_secret)
        .build()
        .expect("Failed to build client");

    // Request an authentication token from Last.fm
    // which later will be exchanged for a session key
    let token = client.get_token().await.expect("Failed to get token");

    // Construct the authorization URL
    // The user must visit this URL and authorize the application
    let auth_url = format!(
        "https://www.last.fm/api/auth/?api_key={}&token={}",
        &api_key, token
    );

    println!("{}", auth_url);

    // Periodically check for approval of the token
    for i in 1..=3 {
        println!("Waiting for approval... Attempt {}/3", i);
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

        // After the user has approved the token, exchange it for a session key
        match client.get_session(&token).await {
            Ok(session) => {
                println!("Session Key for {}: {}", session.name, session.key);
                break;
            }
            Err(e) => {
                if i == 3 {
                    eprintln!("Failed to get session key after 3 attempts: {}", e);
                }
            }
        }
    }
}
