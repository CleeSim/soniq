use std::env;

use soniq::client::Client;
use tokio;

#[tokio::test]
async fn test_user_get_friends() {
    dotenv::dotenv().expect("Failed to load .env file");

    let api_key = env::var("LASTFM_API_KEY").expect("Set LASTFM_API_KEY env var");

    let client = Client::builder(api_key)
        .build()
        .expect("Failed to build client");

    let friends = client.user().get_friends("RJ").await.unwrap();

    assert!(
        friends.attr.total > 0,
        "Expected to find friends, but got none"
    );
}
