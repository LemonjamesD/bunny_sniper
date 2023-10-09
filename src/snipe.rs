use serenity::{framework::standard::macros::group, prelude::Context};

#[command]
pub async fn snipe(ctx: &Context, msg: &Message) {
    msg.channel.say(&ctx.http, format!("Sender: {}\nContext: {}", PAST_MESSAGES.0, PAST_MESSAGES.1)).await?;
}

#[group]
#[commands(snipe)]
pub struct Snipe;
