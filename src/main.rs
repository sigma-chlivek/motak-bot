mod commands;

use crate::commands::message::{handle_dog, handle_roll};
use dotenv::dotenv;
use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const ROLL_COMMAND: &str = "!roll";
const DOG_COMMAND: &str = "!dog";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        match msg.content {
            _ if msg.content.contains(ROLL_COMMAND) => handle_roll(ctx, msg).await,
            _ if msg.content.contains(DOG_COMMAND) => handle_dog(ctx, msg).await.expect("Error"),
            _ => (),
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
