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

#[derive(Clone)]
pub struct MessageType {
    author: User,
    content: String,
    channel_id: u64,
    id: u64
}

impl MessageType {
    pub fn new(author: User, content: String, channel_id: u64, id: u64) -> Self {
        Self {
            author,
            content,
            channel_id,
            id
        }
    }
}

lazy_static! {
    // Hold the past messages
    pub static ref PAST_MESSAGES: Mutex<Vec<MessageType>> = Mutex::new(vec![]);
    pub static ref DELETED_MESSAGES: Mutex<Vec<MessageType>> = Mutex::new(vec![]);
    pub static ref EDITTED_MESSAGES: Mutex<Vec<MessageType>> = Mutex::new(vec![]);
    pub static ref LAST_DELETED_ID: Mutex<u64> = Mutex::new(0);
    pub static ref LAST_EDITTED_ID: Mutex<u64> = Mutex::new(0);
}

// So you can't use up too much memory
const CACHE_CAP: usize = 30;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        let mut locked = PAST_MESSAGES.lock().await;
        // Remove unneeded cached messages
        if locked.len() + 1 >= CACHE_CAP {
            locked.remove(0);
        }
        locked.push(MessageType::new(msg.author, msg.content, msg.channel_id.0, msg.id.0));
    }

    async fn message_delete(&self, _ctx: Context, _channel_id: ChannelId, deleted_message_id: MessageId, _guild_id: Option<GuildId>,) {
        let mut deleted_id = LAST_DELETED_ID.lock().await;
        *deleted_id = deleted_message_id.0;

        // Get the deleted message to add to the deleted_messages stack
        let past_messages = PAST_MESSAGES.lock().await;
        let filtered = past_messages.iter().filter(|m| deleted_message_id.0 == m.id).collect::<Vec<_>>();
        if filtered.len() == 0 {
            return;
        }
        let filtered = filtered[0];

        let mut deleted_messages = DELETED_MESSAGES.lock().await;
        deleted_messages.push(filtered.clone());
    }

    async fn message_update(&self, _ctx: Context, _old_if_available: Option<Message>, new: Option<Message>, event: MessageUpdateEvent) {
        // Push the new editted message onto the message stack
        let mut locked = EDITTED_MESSAGES.lock().await;
        // Remove unneeded cached messages
        if locked.len() + 1 >= CACHE_CAP {
            locked.remove(0);
        }
        // Create the MessageType before hand for less code dupe
        let message_type = match new {
            Some(new) => MessageType::new(new.author.clone(), new.content.clone(), new.channel_id.0, new.id.0),
            None => MessageType::new(event.author.unwrap().clone(), event.content.unwrap().clone(), event.channel_id.0, event.id.0),
        };
        
        locked.push(message_type.clone());
    
        let mut editted_id = LAST_EDITTED_ID.lock().await;
        *editted_id = message_type.id;
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
        StandardFramework::new().configure(|c| c.prefix(",")).group(&SNIPE_GROUP);

    let mut client =
        Client::builder(&TOKEN, intents).framework(framework).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
