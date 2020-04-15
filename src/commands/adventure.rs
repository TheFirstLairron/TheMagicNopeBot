use serenity::prelude::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
pub fn adventure(context: &mut Context, message: &Message) -> CommandResult {
    if let Err(why) = message.channel_id.say(&context.http, "Let's go on an adventure") {
        println!("Error sending Adventure message: {}", why);
    }

    Ok(())
}