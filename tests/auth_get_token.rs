use std::env;

use soniq::client::Client;
use tokio;

#[tokio::test]
async fn test_auth_get_token() {
    dotenv::dotenv().expect("Failed to load .env file");

    let api_key = env::var("LASTFM_API_KEY").expect("Set LASTFM_API_KEY env var");
    let api_secret = env::var("LASTFM_API_SECRET").expect("Set LASTFM_API_SECRET env var");

    let client = Client::builder(api_key)
        .api_secret(api_secret)
        .build()
        .expect("Failed to build client");

    let token = client
        .get_token()
        .await
        .expect("Failed to fetch auth token");

    assert!(!token.is_empty(), "Token should not be empty");
}
