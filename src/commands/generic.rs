use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};



#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;
    Ok(())
}

#[command]
async fn microwave(ctx: &Context, msg: &Message) -> CommandResult {
    let mw = "<:microwave_t_l:754953127345520700><:microwave_t_m:754953127383269396><:microwave_t_r:754953127316160584>\n<:microwave_m_l:754953127429537882><:microwave_m:754953127114833946><:microwave_m_r:754953127102251049>\n<:microwave_b_l:754953127127547974><:microwave_b_m:754953126624100403><:microwave_b_r:754953127089930300>";
    msg.channel_id.say(&ctx.http, &mw).await?;
    Ok(())
}