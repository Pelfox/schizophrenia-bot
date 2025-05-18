//! Contains implementations and definitions for bot handlers.

use teloxide::{
    dispatching::{DpHandlerDescription, HandlerExt, UpdateFilterExt},
    dptree::{self, Handler},
    prelude::{DependencyMap, ResponseResult},
    types::{Message, Update},
};

pub mod dice;
pub mod greeting;
pub mod messages;

/// Setups all available handlers using middlewares and chaining in it.
pub fn setup_handlers() -> Handler<'static, DependencyMap, ResponseResult<()>, DpHandlerDescription>
{
    // commands handler
    let command_branch = dptree::entry()
        .filter_command::<crate::commands::Command>()
        .endpoint(crate::commands::handle_command);

    // messages handler
    let messages_branch = dptree::entry()
        .filter(|message: Message| message.text().is_some())
        .endpoint(messages::save_and_answer);

    // dices handler
    let dice_branch = dptree::entry()
        .filter_map(|msg: Message| msg.dice().cloned())
        .endpoint(dice::dice_handler);

    // greeting handler
    let greeting_branch = dptree::entry()
        .filter_map(|msg: Message| msg.new_chat_members().map(|v| v.to_vec()))
        .endpoint(greeting::greeting_handler);

    Update::filter_message()
        .branch(command_branch)
        .branch(messages_branch)
        .branch(dice_branch)
        .branch(greeting_branch)
}
