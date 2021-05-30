use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use crate::config::Config;

use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::{
    async_trait,
    model::{
        channel::{Channel, Reaction, ReactionType},
        prelude::Ready,
    },
};
use serenity::{
    client::{Cache, Client, Context, EventHandler},
    framework::standard::macros::hook,
    http::Http,
    model::channel::GuildChannel,
    prelude::TypeMapKey,
    CacheAndHttp,
};

use tokio::{stream, time::Instant};
use tracing::{debug, error, info, instrument};

#[group]
#[commands(ping)]
struct General;

struct Handler;

struct DusdatCount {}

impl TypeMapKey for DusdatCount {
    type Value = Arc<AtomicUsize>;
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} connected", ready.user.name);
    }

    async fn message(&self, ctx: Context, message: Message) {
        if *message.channel_id.as_u64() == 848489433464438825
            && message.content.to_ascii_lowercase().contains("kut")
        {
            let msg = message
                .channel_id
                .send_message(ctx, |m| {
                    m.content("Het komt wel goed schatje");
                    m
                })
                .await;
            if let Err(why) = msg {
                error!("Error sending message: {:?}", why);
            }
        }
    }
}

#[hook]
#[instrument]
async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
    info!(
        "Got command '{}' by user '{}'",
        command_name, msg.author.name
    );

    true
}

#[instrument]
pub async fn start(config: Config) {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("&"))
        .before(before)
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let mut client = Client::builder(config.token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }

    info!("help2");
    println!("Bot online");
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
