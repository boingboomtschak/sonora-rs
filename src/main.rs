mod commands;
mod messages;

use std::env;
use serenity::{
    async_trait,
    client::{Client, Context},
    model::{channel::Message, gateway::Ready},
    framework::{
        StandardFramework,
        standard::macros::{
            hook,
            group,
        },
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
#[commands(ping, microwave, fmicrowave)]
struct General;

#[hook]
async fn before(_ctx: &Context, msg: &Message, command_name: &str) -> bool {
    println!("Got command '{}' from user '{}'", command_name, msg.author.name);
    true
}

#[tokio::main]
async fn main() {
    println!("Starting up bot...");
    dotenv::dotenv().expect("Failed to load .env file!");
    println!("Loading token from dotenv...");
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