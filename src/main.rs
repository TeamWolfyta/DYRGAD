mod events;

use serenity::all::{ClientBuilder, GatewayIntents};
use std::env;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();

  let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
  let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

  let mut client = ClientBuilder::new(&token, intents)
    .event_handler(events::EventHandler {})
    .await
    .unwrap();

  client.start().await.unwrap();
}
