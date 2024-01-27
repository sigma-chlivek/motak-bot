extern crate rand;

use rand::Rng;
use serenity::{model::channel::Message, prelude::*};

fn parse_numeric(input_str: &str) -> Option<i32> {
    match input_str.parse::<i32>() {
        Ok(number) => Some(number),
        Err(_) => None,
    }
}
pub async fn handle_roll(ctx: Context, msg: Message) {
    let roll_command_parts: Vec<&str> = msg.content.split_whitespace().collect();

    let first_part = roll_command_parts
        .get(1)
        .map(|&s| parse_numeric(s))
        .unwrap_or(None);
    let second_part = roll_command_parts
        .get(2)
        .map(|&s| parse_numeric(s))
        .unwrap_or(None);

    let roll_number = match (first_part, second_part) {
        (Some(end), None) => rand::thread_rng().gen_range(0..end),
        (Some(start), Some(end)) => rand::thread_rng().gen_range(start..end),
        _ => rand::thread_rng().gen_range(0..100),
    };

    let message = if roll_number == 69 {
        "Nice".to_string()
    } else {
        roll_number.to_string()
    };

    if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
        println!("Error sending message: {:?}", why);
    }
}
