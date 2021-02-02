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
#[aliases("h")]
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
#[aliases("remind", "rm")]
async fn remindme(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Instantiate Reminder struct with time, message, and recipients
    let mut reminder = Reminder {
        time: None,
        message: None,
        recip: None,
    };
    if args.is_empty() {
        msg.channel_id.say(&ctx.http, "Missing reminder time.").await?;
    } else {
        reminder.time = Some(Reminder::parse_time(args.single::<String>().unwrap()).unwrap());
    }
    if args.is_empty() {
        msg.channel_id.say(&ctx.http, "Missing reminder message.").await?;
    } else {
        reminder.message = Some(args.single_quoted::<String>().unwrap());
    }
    if args.is_empty() {
        reminder.recip = Some(String::from("channel"));
    } else {
        reminder.recip = Some(args.single::<String>().unwrap());
    }
    let response = format!("Reminder - time: `{}`, message: `{}`, recipient: `{}`", reminder.time.unwrap(), reminder.message.unwrap(), reminder.recip.unwrap());
    if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
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