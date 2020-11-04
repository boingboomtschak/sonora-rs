use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
        group,
    },
};

#[group]
#[commands(help, ping, remindme)]
pub struct Generic;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, crate::messages::HELP_MESSAGE).await {
        println!("Error running 'help' command: {:?}", why);
    }
    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Error running 'ping' command: {:?}", why);
    }
    Ok(())
}

#[command]
async fn remindme(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Reminder set to `do something` at `yyyy/mm/dd hh:mm`").await {
        println!("Error running 'remindme' command: {:?}", why);
    }
    Ok(())
}