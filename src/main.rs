pub mod secrets;
pub mod snipe;

use crate::snipe::SNIPE_GROUP;
use secrets::TOKEN;

use serenity::async_trait;
use serenity::framework::StandardFramework;
use serenity::prelude::*;
use serenity::model::prelude::*;

use lazy_static::lazy_static;
use tracing_subscriber::fmt::init;

struct Handler;

// Hold the past messages
lazy_static! {
    pub static ref PAST_MESSAGES: Mutex<Vec<(String, String, u64)>> = Mutex::new(vec![]);
    pub static ref LAST_DELETED_ID: Mutex<u64> = Mutex::new(0);
    pub static ref LAST_EDITTED_ID: Mutex<u64> = Mutex::new(0);
}

// So you can't use up too much memory
const CACHE_CAP: usize = 10;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let mut locked = PAST_MESSAGES.lock().await;
        // Remove unneeded cached messages
        if locked.len() + 1 >= CACHE_CAP {
            locked.remove(0);
        }
        locked.push((msg.author.name.clone(), msg.content.clone(), msg.id.0));
    }

    async fn message_delete(&self, _ctx: Context, _channel_id: ChannelId, deleted_message_id: MessageId, _guild_id: Option<GuildId>,) {
        let mut locked = LAST_DELETED_ID.lock().await;
        *locked = deleted_message_id.0;
    }

    async fn message_update(&self, _ctx: Context, _old_if_available: Option<Message>, new: Option<Message>, event: MessageUpdateEvent) {
        // Push the new editted message onto the message stack
        let mut locked = PAST_MESSAGES.lock().await;
        // Remove unneeded cached messages
        if locked.len() + 1 >= CACHE_CAP {
            locked.remove(0);
        }
        if let Some(new) = new {
            locked.push((new.author.name.clone(), new.content.clone(), new.id.0));
        
            let mut locked = LAST_EDITTED_ID.lock().await;
            *locked = new.id.0;
        } else {
            locked.push((event.author.unwrap().name.clone(), event.content.unwrap().clone(), event.id.0));
            
            let mut locked = LAST_EDITTED_ID.lock().await;
            *locked = event.id.0;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Init logging
    init();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework =
        StandardFramework::new().configure(|c| c.prefix(".")).group(&SNIPE_GROUP);

    let mut client =
        Client::builder(&TOKEN, intents).framework(framework).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
