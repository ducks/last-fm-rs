use last_fm_rs::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Get API key from environment
  let api_key = std::env::var("LASTFM_API_KEY").expect("LASTFM_API_KEY not set");
  let api_secret = std::env::var("LASTFM_API_SECRET").unwrap_or_else(|_| "secret".to_string());

  let client = Client::new(api_key, api_secret);

  // Get track info
  println!("Getting info for 'Wesley's Theory' by Kendrick Lamar...\n");

  let track = client
    .track_get_info("Kendrick Lamar", "Wesley's Theory", None)
    .await?;

  println!("Track: {}", track.name);
  println!("Artist: {}", track.artist.name);
  println!("URL: {}", track.url);
  println!("Listeners: {}", track.listeners);
  println!("Playcount: {}", track.playcount);

  if let Some(duration) = track.duration {
    println!("Duration: {}s", duration / 1000);
  }

  if let Some(album) = &track.album {
    println!("\nAlbum: {} by {}", album.title, album.artist);
    if let Some(image) = album.image.last() {
      println!("Cover art: {}", image.url);
    }
  }

  if let Some(tags) = &track.toptags {
    if !tags.tag.is_empty() {
      println!("\nTop tags:");
      for tag in tags.tag.iter().take(5) {
        println!("  - {}", tag.name);
      }
    }
  }

  if let Some(wiki) = &track.wiki {
    println!("\nWiki summary:");
    // Strip HTML tags for cleaner display
    let summary = wiki.summary.replace("<a href=", "\n  Link: ");
    println!("{}", summary.chars().take(200).collect::<String>());
    println!("...");
  }

  Ok(())
}
