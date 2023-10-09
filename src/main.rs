pub mod secrets;
pub mod snipe;

use secrets::TOKEN;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::prelude::*;

use lazy_static::lazy_static;
use tracing_subscriber::fmt::init;

struct Handler;

// Hold the past messages
lazy_static! {
    pub static ref PAST_MESSAGES: Mutex<Vec<(&'static str, &'static str)>> = Mutex::new(vec![("No one", "None")]);
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

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
        StandardFramework::new().configure(|c| c.prefix(".")).group(&SNIPE);

    let mut client =
        Client::builder(&TOKEN, intents).framework(framework).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
