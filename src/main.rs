use serenity::framework::StandardFramework;
use serenity::prelude::GatewayIntents;
use serenity::Client;

mod commands;
mod handler;
mod settings;

const DISCORD_TOKEN: &str =
    "MTEwNDEyMTUzMTI0OTk5MTc0MQ.GC1hcl.Zdzs1KRqZ-3KAHUZKaUDrp4U921_OKr3y6xN7c";

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
