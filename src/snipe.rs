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

// This might be unnecessary but they're dupes of each other so why not
macro_rules! snipe {
    ($fn_name:ident, $cache:ident, $id:ident) => {
        #[command]
        pub async fn $fn_name(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
            let locked = $cache.lock().await;
            let id_locked = $id.lock().await;

            // Check if there are deleted messages and if there is none then say there is none
            if locked.len() == 0 || *id_locked == 0 {
                msg.channel_id.say(&ctx.http, "There are no cached messages").await?;
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
    }
}

snipe!(snipe, DELETED_MESSAGES, LAST_DELETED_ID);
snipe!(edit_snipe, EDITTED_MESSAGES, LAST_EDITTED_ID);

#[group]
#[commands(edit_snipe, snipe)]
pub struct Snipe;
