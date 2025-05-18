//! Contains handlers for all greeting-related functionality.

use std::sync::Arc;

use teloxide::{
    prelude::{Requester, ResponseResult},
    types::{Message, User},
};

use crate::{bot::Bot, modules::i18n::I18nLanguage};

/// Handles the actions taken when the bot is added.
pub async fn greeting_handler(
    bot: Bot,
    message: Message,
    members: Vec<User>,
    language: Arc<I18nLanguage>,
) -> ResponseResult<()> {
    let my_id = bot.get_me().await?.user.id;
    let is_myself_added = members.iter().any(|m| m.id == my_id);

    // trigger only if the self-bot was added
    if !is_myself_added {
        return Ok(());
    }

    bot.send_message(message.chat.id, &language.greeting_message)
        .await?;
    Ok(())
}
