use serenity::{model::channel::Message, prelude::*};

pub fn parse_numeric(input_str: &str) -> Option<i32> {
    match input_str.parse::<i32>() {
        Ok(number) => Some(number),
        Err(_) => None,
    }
}
