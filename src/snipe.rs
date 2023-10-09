use serenity::framework::standard::{Args, CommandResult};
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::macros::{command, group};

use crate::{PAST_MESSAGES, LAST_DELETED_ID};

#[command]
pub async fn snipe(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let mut locked = PAST_MESSAGES.lock().await;
    let id_locked = LAST_DELETED_ID.lock().await;
    let filtered = locked.iter().filter(|(author, content, id)| *id == *id_locked).collect::<Vec<_>>()[0];
    msg.channel_id.say(&ctx.http, format!("Sender: {}\nContext: {}\nId: {}", filtered.0, filtered.1, filtered.2)).await?;

    Ok(())
}

#[group]
#[commands(snipe)]
pub struct Snipe;
