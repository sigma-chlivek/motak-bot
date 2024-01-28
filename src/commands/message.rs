extern crate rand;

use crate::commands::utils::{generate_random_number, handle_error, parse_numeric};
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

    let roll_number_res = match (lower_limit, upper_limit) {
        (Some(end), None) => generate_random_number(0, end),
        (Some(start), Some(end)) => generate_random_number(start, end),
        _ => generate_random_number(0, 100),
    };

    let roll_number = match roll_number_res {
        Ok(num) => num,
        Err(_) => {
            handle_error(&ctx, &msg, "Error while generating random number").await;
            return;
        }
    };

    let message = match roll_number {
        69 => "Nice (69)".to_string(),
        420 => "Michal is blazing (420)".to_string(),
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
