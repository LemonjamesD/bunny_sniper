use serenity::framework::standard::{Args, CommandResult};
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::macros::{command, group};

use crate::PAST_MESSAGES;

#[command]
pub async fn snipe(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let mut locked = PAST_MESSAGES.lock().await;
    msg.channel_id.say(&ctx.http, format!("Sender: {}\nContext: {}", locked[0].0, locked[0].1)).await?;

    Ok(())
}

#[group]
#[commands(snipe)]
pub struct Snipe;
