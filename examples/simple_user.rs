use std::env;

use soniq::client::Client;

#[tokio::main]
async fn main() {
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

    println!(
        "{} has {} total plays ({})",
        user.name, user.playcount, user.url
    );

    let friends = client.user().get_friends("unb_").await;

    if friends.is_err() {
        println!(
            "couldn't find any friends for {} (err: {:?})",
            user.name, friends
        )
    } else {
        println!(
            "{} has {} total friends",
            user.name,
            friends.unwrap().attr.total
        );
    }
}
