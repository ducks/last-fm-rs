/// Example: Desktop authentication flow
///
/// Usage:
///   cargo run --example auth -- YOUR_API_KEY YOUR_API_SECRET

use last_fm_rs::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args: Vec<String> = std::env::args().collect();

  if args.len() != 3 {
    eprintln!("Usage: {} API_KEY API_SECRET", args[0]);
    std::process::exit(1);
  }

  let api_key = &args[1];
  let api_secret = &args[2];

  println!("Last.fm Desktop Authentication Example\n");

  let client = Client::new(api_key, api_secret);

  // Step 1: Get token
  println!("Step 1: Requesting authentication token...");
  let token = client.get_token().await?;
  println!("✓ Token received: {}\n", token.token);

  // Step 2: Direct user to authorize
  let auth_url = client.get_auth_url(&token)?;
  println!("Step 2: Please authorize this application:");
  println!("  {}\n", auth_url);
  println!("Press Enter after you've authorized...");

  let mut input = String::new();
  std::io::stdin().read_line(&mut input)?;

  // Step 3: Exchange token for session key
  println!("Step 3: Exchanging token for session key...");
  let session = client.get_session(&token).await?;
  println!("✓ Authentication successful!\n");

  println!("Session details:");
  println!("  Username: {}", session.name);
  println!("  Session key: {}", session.key);
  println!("\nSave this session key for future use!");

  Ok(())
}
