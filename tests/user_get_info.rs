use std::env;

use soniq::client::Client;
use tokio;

#[tokio::test]
async fn test_user_get_info() {
    dotenv::dotenv().expect("Failed to load .env file");

    let api_key = env::var("LASTFM_API_KEY").expect("Set LASTFM_API_KEY env var");

    let client = Client::builder(api_key)
        .build()
        .expect("Failed to build client");

    let user = client
        .user()
        .get_info("unb_")
        .await
        .expect("Failed to fetch user info");

    assert_eq!(
        user.realname
            .unwrap_or("n/a".to_string())
            .to_ascii_lowercase(),
        "iku"
    );
    assert!(user.playcount > 0, "Playcount should be greater than zero");
}
