mod commands;

use serenity::client::Client;
use serenity::model::gateway::Ready;
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
    StandardFramework,
    macros::group
};

use commands::{
    meta::*,
    adventure::*,
    hr::*,
};

#[group]
#[commands(ping, adventure)]
struct General;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _:Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}



fn main() {
    let token = include_str!("API.KEY");

    let mut client = Client::new(&token, Handler).expect("Error Creating Client.");

    client.with_framework(StandardFramework::new()
                        .configure(|c| c.prefix("!"))
                        .group(&GENERAL_GROUP));

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
