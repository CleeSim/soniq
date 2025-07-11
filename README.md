# Soniq

![Crates.io](https://img.shields.io/crates/v/soniq)
![Docs.rs](https://docs.rs/soniq/badge.svg)
![License](https://img.shields.io/crates/l/soniq)

A modern and lightweight Rust client for the [Last.fm API](https://www.last.fm/api). Fully asynchronous and built with Tokio, Soniq provides a simple and efficient way to interact with Last.fm's extensive music data.

## Installation

```bash
cargo add soniq
```

Or add Soniq to your `Cargo.toml` file directly:

```toml
[dependencies]
soniq = "*.*.*"
```

## Quick Start

```rust
use soniq::client::Client;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = "YOUR_LASTFM_API_KEY";
    let client = Client::builder(api_key.clone())
        .api_secret("YOUR_LASTFM_API_SECRET") // optional for signed calls
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    let user_info = client
        .user()
        .get_info("unb_")
        .await?;

    println!("User: {:?}", user_info);

    Ok(())
}
```

## Todo

The library is still in early development. Here are some of the planned features and improvements:

- [ ] Add missing user methods
- [ ] Implement album methods
- [ ] Implement artist methods
- [x] Implement auth methods
- [ ] Implement chart methods
- [ ] Implement geo methods
- [ ] Implement library methods
- [ ] Implement tag methods
- [ ] Implement track methods

## Contributing

Contributions, issues, and feature requests are welcome!

## License

This project is licensed under the MIT License.
See [LICENSE](LICENSE) for details.
