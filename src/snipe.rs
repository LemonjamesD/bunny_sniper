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
        pub async fn $fn_name(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
            // First block for args part
            if args.len() >= 1 {
                let idx = match args.single::<usize>() {
                    Ok(ok) => ok,
                    Err(_) => {
                        msg.channel_id.say(&ctx.http, "That is not a valid input for the index of the deleted messages").await?;
                        return Ok(());
                    }
                };

                let locked = $cache.lock().await;
                let filtered = locked.iter().filter(|m| m.channel_id == msg.channel_id.0).collect::<Vec<_>>();
                let filtered = match filtered.get(filtered.len() - 1 - idx) {
                    Some(some) => some,
                    _ => return Ok(()),
                };
                
                msg.channel_id.send_message(&ctx.http, |m| {
                    m.add_embed(|e| {
                        e.author(|a| {
                            match filtered.author.avatar_url().clone() {
                                Some(av) => a.icon_url(av).name(filtered.author.name.clone()),
                                None => a.name(filtered.author.name.clone())
                            }
                         })
                         .field("Message", filtered.content.clone(), false)
                         .color(random_color())
                    })
                }).await?;
            } else {
                let locked = $cache.lock().await;
                let id_locked = $id.lock().await;

                // Check if there are deleted messages and if there is none then say there is none
                if locked.len() == 0 || *id_locked == 0 {
                    msg.channel_id.say(&ctx.http, "There are no cached messages").await?;
                    return Ok(());
                }
                let filtered = locked.iter().filter(|m| m.channel_id == msg.channel_id.0 && m.id == *id_locked).collect::<Vec<_>>();
                if filtered.len() == 0 {
                    msg.channel_id.say(&ctx.http, "There are no cached messages").await?;
                    return Ok(());
                }
                let filtered = filtered[filtered.len() - 1];
                msg.channel_id.send_message(&ctx.http, |m| {
                    m.add_embed(|e| {
                        e.author(|a| {
                            match filtered.author.avatar_url().clone() {
                                Some(av) => a.icon_url(av).name(filtered.author.name.clone()),
                                None => a.name(filtered.author.name.clone())
                            }
                         })
                         .field("Message", filtered.content.clone(), false)
                         .color(random_color())
                    })
                }).await?;
            }
            Ok(())
        }
    }
}

snipe!(snipe, DELETED_MESSAGES, LAST_DELETED_ID);
snipe!(edit_snipe, EDITTED_MESSAGES, LAST_EDITTED_ID);

#[group]
#[commands(edit_snipe, snipe)]
pub struct Snipe;
