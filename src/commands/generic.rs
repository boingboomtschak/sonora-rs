use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
        group,
    },
    Args
};

use crate::utils::reminder::*;

#[group]
#[commands(help, ping, remindme, embed)]
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
async fn remindme(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let reminder = Reminder {
        time: args.single::<String>().unwrap(),
        message: args.single::<String>().unwrap()
    };
    if let Err(why) = msg.channel_id.say(&ctx.http, format!("Reminder - time: `{}`, message: `{}`", reminder.time, reminder.message)).await {
        println!("Error running 'remindme' command: {:?}", why);
    }
    Ok(())
}

#[command]
async fn embed(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m | {
        m.embed(|e | {
            e.title("This is a cat!");
            e.description("This is a description of the cat!");
            return e;
        });
        return m;
    }).await {
        println!("Error sending embed: {}", why);
    }
    Ok(())
}