use serenity::framework::standard::{Args, CommandResult};
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::macros::{command, group};

use rand::{Rng, thread_rng};

use crate::{LAST_DELETED_ID, LAST_EDITTED_ID, EDITTED_MESSAGES, DELETED_MESSAGES};

fn random_color() -> u32 {
    let mut rng = thread_rng();

    rng.gen_range(0..16777215)
}

#[command]
pub async fn snipe(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let mut locked = DELETED_MESSAGES.lock().await;
    let id_locked = LAST_DELETED_ID.lock().await;
    // Check if there are deleted messages and if there is none then say there is none
    if locked.len() == 0 || *id_locked == 0 {
        msg.channel_id.say(&ctx.http, "There are no cached deleted messages").await?;
        return Ok(());
    }
    let filtered = locked.iter().filter(|m| m.id == *id_locked).collect::<Vec<_>>()[0];
    msg.channel_id.send_message(&ctx.http, |m| {
        m.add_embed(|e| {
            e.field("Author", filtered.author.clone(), false)
             .field("Message", filtered.content.clone(), false)
             .color(random_color())
        })
    }).await?;

    Ok(())
}

// Basically the same thing as the snipe command but get edits
#[command]
pub async fn edit_snipe(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let mut locked = EDITTED_MESSAGES.lock().await;
    let id_locked = LAST_EDITTED_ID.lock().await;
    // Check if there are deleted messages and if there is none then say there is none
    if locked.len() == 0 || *id_locked == 0 {
        msg.channel_id.say(&ctx.http, "There are no cached deleted messages").await?;
        return Ok(());
    }
    let filtered = locked.iter().filter(|m| m.id == *id_locked).collect::<Vec<_>>();
    let filtered = filtered[filtered.len() - 1];
    msg.channel_id.send_message(&ctx.http, |m| {
        m.add_embed(|e| {
            e.field("Author", filtered.author.clone(), false)
             .field("Message", filtered.content.clone(), false)
             .color(random_color())
        })
    }).await?;

    Ok(())
}

#[group]
#[commands(edit_snipe, snipe)]
pub struct Snipe;
