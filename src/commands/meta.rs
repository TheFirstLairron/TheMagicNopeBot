use serenity::prelude::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
pub fn ping(context: &mut Context, message: &Message) -> CommandResult {
    message.reply(context, "Pong!")?;
    
    Ok(())
}