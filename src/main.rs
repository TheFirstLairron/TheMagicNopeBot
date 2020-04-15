use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct ResponseHandler;

impl EventHandler for ResponseHandler {
    fn message(&self, context: Context, message: Message) {
        if message.content == "!ping" {
            if let Err(why) = message.channel_id.say(&context.http, "Pong!") {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, _context: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    let token = include_str!("API.KEY");

    let mut client = Client::new(&token, ResponseHandler).expect("Error creating client");

    if let Err(why) = client.start() {
        println!("Client Error: {:?}", why);
    }
}