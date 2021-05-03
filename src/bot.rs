use crate::config::Config;

use serenity::client::{Client, Context, EventHandler};
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

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        reaction_handler(ctx, reaction).await.unwrap()
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} connected", ready.user.name);
    }
}

pub async fn start(config: Config) {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("&"))
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

    println!("Bot online");
}

async fn reaction_handler(ctx: Context, reaction: Reaction) -> Result<(), anyhow::Error> {
    println!("reaction received");
    if let Some(guild_id) = reaction.guild_id {
        if guild_id.to_string() == "834904540590899200" {
            if let ReactionType::Custom { name, .. } = reaction.emoji {
                dusdat_counter(ctx, name).await?;
            }
        }
    }

    Ok(())
}

async fn dusdat_counter(ctx: Context, emoji: Option<String>) -> Result<(), anyhow::Error> {
    if let Some(name) = emoji {
        if name == "dusdat" {
            let channel = ctx.http.get_channel(838781927578009672).await?;
            if let Channel::Guild(mut channel) = channel {
                let name = channel.name.clone();
                let count: i32 = name.split(' ').last().unwrap().parse()?;
                let channel_name = format!("Dusdats: {}", count + 1);
                channel.edit(ctx, |c| c.name(channel_name)).await?;
            }
        }
    }

    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
