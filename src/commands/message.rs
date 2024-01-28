extern crate rand;

use crate::commands::utils::parse_numeric;
use rand::Rng;
use serenity::all::standard::CommandResult;
use serenity::{model::channel::Message, prelude::*};
use std::collections::HashMap;

const DOG_API_URL: &str = "https://dog.ceo/api/breeds/image/random";

pub async fn handle_roll(ctx: Context, msg: Message) {
    let roll_command_parts: Vec<&str> = msg.content.split_whitespace().collect();

    let lower_limit = roll_command_parts
        .get(1)
        .map(|&s| parse_numeric(s))
        .unwrap_or(None);
    let upper_limit = roll_command_parts
        .get(2)
        .map(|&s| parse_numeric(s))
        .unwrap_or(None);

    let roll_number = match (lower_limit, upper_limit) {
        (Some(end), None) => rand::thread_rng().gen_range(0..end),
        (Some(start), Some(end)) => {
            if start > end {
                rand::thread_rng().gen_range(end..start)
            } else {
                rand::thread_rng().gen_range(start..end)
            }
        }
        _ => rand::thread_rng().gen_range(0..100),
    };

    let message = match roll_number {
        69 => "Nice (69)".to_string(),
        420 => "Michal is blazing".to_string(),
        _ => roll_number.to_string(),
    };

    if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
        println!("Error sending message: {:?}", why);
    }
}

pub async fn handle_dog(ctx: Context, msg: Message) -> CommandResult {
    let response = reqwest::get(DOG_API_URL).await?;
    let data = response.json::<HashMap<String, String>>().await?;

    let message = match data.get("message") {
        Some(value) => value.to_string(),
        None => "None".to_string(),
    };

    if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
