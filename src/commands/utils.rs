use rand::Rng;
use serenity::{model::channel::Message, prelude::*};

pub fn parse_numeric(input_str: &str) -> Option<i32> {
    match input_str.parse::<i32>() {
        Ok(number) => Some(number),
        Err(_) => None,
    }
}

pub fn generate_random_number(start: i32, end: i32) -> Result<i32, &'static str> {
    if start >= end {
        return Err("Error while generating random number");
    }
    Ok(rand::thread_rng().gen_range(start..end))
}

pub async fn handle_error(ctx: &Context, msg: &Message, message: &str) {
    let message = format!("***ERROR*** - {}", message);
    println!("{}", message.replace("*", ""));

    if let Err(why) = msg.channel_id.say(&ctx.http, &message).await {
        println!("Error sending message: {:?}", why);
    }
}
