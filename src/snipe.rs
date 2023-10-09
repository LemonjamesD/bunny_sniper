use serenity::{framework::standard::macros::group, prelude::Context};

#[command]
pub async fn snipe(ctx: &Context, msg: &Message) {
    let locked = PAST_MESSAGES.lock();
    msg.channel.say(&ctx.http, format!("Sender: {}\nContext: {}", locked[0].0, locked[0].1)).await?;
}

#[group]
#[commands(snipe)]
pub struct Snipe;
