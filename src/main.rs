mod commands;

use std::env;
use serenity::{
    async_trait,
    client::{Client, Context},
    model::{channel::Message, gateway::Ready},
    framework::{
        StandardFramework,
        standard::macros::group,
    },
    prelude::*,
};

use commands::{
    generic::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}

#[group]
#[commands(ping)]
struct General;

#[tokio::main]
async fn main() {
    println!("Starting up bot...");
    let token = env::var("DISCORD_TOKEN").expect("Expected 'DISCORD_TOKEN' in envvars!");
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);
    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client!");
    println!("Started up client!");
    println!("Listening for events...");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}