//! Implementation for a `/start` command.

use std::sync::Arc;

use teloxide::{
    Bot,
    adaptors::DefaultParseMode,
    prelude::{Requester, ResponseResult},
    types::Message,
};

use crate::modules::i18n::I18nLanguage;

/// Handles `/start` command sent by user.
pub async fn handle_start(
    bot: DefaultParseMode<Bot>,
    message: Message,
    language: Arc<I18nLanguage>,
) -> ResponseResult<()> {
    bot.send_message(message.chat.id, &language.start_message)
        .await?;
    Ok(())
}
