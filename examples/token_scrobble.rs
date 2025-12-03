/// Example: Token-based scrobbling to custom server
///
/// This example shows how to use the token-based authentication mode
/// to scrobble to your own self-hosted scrobbling server.
///
/// Usage:
///   cargo run --example token_scrobble -- BASE_URL TOKEN

use last_fm_rs::{Client, NowPlaying, Scrobble};
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args: Vec<String> = std::env::args().collect();

  if args.len() != 3 {
    eprintln!("Usage: {} BASE_URL TOKEN", args[0]);
    eprintln!("\nExample:");
    eprintln!("  {} https://scrob.example.com/api/ my-secret-token", args[0]);
    std::process::exit(1);
  }

  let base_url = &args[1];
  let token = &args[2];

  println!("Token-based Scrobbling Example\n");

  // Create client with token-based auth
  println!("Creating client for: {}", base_url);
  let client = Client::with_token(base_url, token)?;
  println!("✓ Client created\n");

  // Update "Now Playing"
  println!("Updating Now Playing...");
  let now_playing = NowPlaying::new("Kendrick Lamar", "Wesley's Theory")
    .with_album("To Pimp a Butterfly")
    .with_duration(287)
    .with_track_number(1);

  client.update_now_playing(&now_playing).await?;
  println!("✓ Now Playing updated\n");

  // Submit a scrobble
  println!("Submitting scrobble...");
  let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)?
    .as_secs();

  let scrobble = Scrobble::new("Kendrick Lamar", "Wesley's Theory", timestamp)
    .with_album("To Pimp a Butterfly")
    .with_duration(287)
    .with_track_number(1);

  let response = client.scrobble(&[scrobble]).await?;
  println!("✓ Scrobble submitted successfully!");
  println!("  Accepted: {}", response.scrobbles.attr.accepted);
  println!("  Ignored: {}\n", response.scrobbles.attr.ignored);

  // Submit multiple scrobbles (batch)
  println!("Submitting batch scrobbles...");
  let timestamp2 = timestamp - 300;
  let timestamp3 = timestamp - 600;

  let scrobbles = vec![
    Scrobble::new("Pink Floyd", "Time", timestamp2)
      .with_album("The Dark Side of the Moon")
      .with_track_number(4),
    Scrobble::new("Pink Floyd", "The Great Gig in the Sky", timestamp3)
      .with_album("The Dark Side of the Moon")
      .with_track_number(5),
  ];

  let response = client.scrobble(&scrobbles).await?;
  println!("✓ Batch scrobbles submitted successfully!");
  println!("  Accepted: {}", response.scrobbles.attr.accepted);
  println!("  Ignored: {}\n", response.scrobbles.attr.ignored);

  println!("All operations completed successfully!");

  Ok(())
}
