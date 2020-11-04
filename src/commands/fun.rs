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
#[commands(microwave, fmicrowave)]
pub struct Fun;

#[command]
async fn microwave(ctx: &Context, msg: &Message) -> CommandResult {
    let mw = "<:microwave_t_l:754953127345520700><:microwave_t_m:754953127383269396><:microwave_t_r:754953127316160584>\n<:microwave_m_l:754953127429537882><:microwave_m:754953127114833946><:microwave_m_r:754953127102251049>\n<:microwave_b_l:754953127127547974><:microwave_b_m:754953126624100403><:microwave_b_r:754953127089930300>";
    if let Err(why) = msg.channel_id.say(&ctx.http, &mw).await {
        println!("Error running 'microwave' command: {:?}", why);
    }
    Ok(())
}

#[command]
async fn fmicrowave(ctx: &Context, msg: &Message) -> CommandResult {
    let mw = "<:fmicrowave_t_l:755193429402714122><:fmicrowave_t_m:755193429339930634><:fmicrowave_t_r:755193432347246722>\n<:fmicrowave_m_l:755193429121564714><:fmicrowave_m:755193429213970463><:fmicrowave_m_r:755193429444526211>\n<:fmicrowave_b_l:755193429280948315><:fmicrowave_b_m:755193429243199518><:fmicrowave_b_r:755193429268365423>";
    if let Err(why) = msg.channel_id.say(&ctx.http, &mw).await {
        println!("Error running 'fmicrowave' command: {:?}", why);
    }
    Ok(())
}