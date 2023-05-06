#![feature(async_closure)]

use serenity::framework::StandardFramework;
use serenity::prelude::GatewayIntents;
use serenity::Client;

mod commands;
mod handler;
mod settings;

const DISCORD_TOKEN: &str =
    "MTEwNDEyMTUzMTI0OTk5MTc0MQ.GnK824._b-mowvaOoidTDF4j6ufH2jtQqwCfn4EYyA5Jk";

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new().configure(|c| c.prefix(">>"));
    let mut client = Client::builder(DISCORD_TOKEN, GatewayIntents::all())
        .event_handler(handler::Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
