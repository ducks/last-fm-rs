# last-fm-rs

Rust client library for the Last.fm API, focused on desktop application scrobbling.

## Features

- Desktop authentication flow
- "Now Playing" updates
- Scrobble submission (single or batch up to 50)
- Fully async with tokio
- Type-safe API

## Installation

```toml
[dependencies]
last-fm-rs = "0.1"
```

## Usage

### Authentication Flow

```rust
use last_fm_rs::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Create client with your API credentials
  let client = Client::new("your_api_key", "your_api_secret");

  // Step 1: Get authentication token
  let token = client.get_token().await?;

  // Step 2: Direct user to authorize
  let auth_url = client.get_auth_url(&token);
  println!("Please authorize at: {}", auth_url);
  println!("Press Enter when done...");

  let mut input = String::new();
  std::io::stdin().read_line(&mut input)?;

  // Step 3: Exchange token for session key
  let session = client.get_session(&token).await?;
  println!("Session key: {}", session.key);
  println!("Username: {}", session.name);

  // Save session.key for future use
  Ok(())
}
```

### Scrobbling

```rust
use last_fm_rs::{Client, NowPlaying, Scrobble};
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = Client::new("api_key", "secret")
    .with_session_key("saved_session_key");

  // Update "Now Playing"
  let now_playing = NowPlaying::new("Kendrick Lamar", "Wesley's Theory")
    .with_album("To Pimp a Butterfly")
    .with_duration(287);

  client.update_now_playing(&now_playing).await?;

  // Submit scrobble after track finishes
  let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)?
    .as_secs();

  let scrobble = Scrobble::new("Kendrick Lamar", "Wesley's Theory", timestamp)
    .with_album("To Pimp a Butterfly")
    .with_duration(287);

  let response = client.scrobble(&[scrobble]).await?;
  println!("Scrobbled: {} accepted, {} ignored",
    response.scrobbles.attr.accepted,
    response.scrobbles.attr.ignored
  );

  Ok(())
}
```

### Batch Scrobbling

```rust
let scrobbles = vec![
  Scrobble::new("Artist 1", "Track 1", timestamp1),
  Scrobble::new("Artist 2", "Track 2", timestamp2),
  // ... up to 50 scrobbles
];

let response = client.scrobble(&scrobbles).await?;
```

## API Credentials

Get your API key and secret from: https://www.last.fm/api/account/create

## License

MIT OR Apache-2.0
