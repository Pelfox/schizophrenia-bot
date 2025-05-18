//! Implementation for a `/stats` command.

use std::sync::Arc;

use log::error;
use teloxide::{
    Bot,
    adaptors::DefaultParseMode,
    prelude::{Requester, ResponseResult},
    types::Message,
};

use crate::{
    models::message::count_messages_by_chat,
    modules::{database::PgPool, i18n::I18nLanguage},
};

/// Handles `/stats` command sent by user.
pub async fn handle_stats(
    bot: DefaultParseMode<Bot>,
    message: Message,
    pool: Arc<PgPool>,
    language: Arc<I18nLanguage>,
) -> ResponseResult<()> {
    let messages_count = match count_messages_by_chat(pool, message.chat.id.0).await {
        Ok(count) => count,
        Err(e) => {
            error!(target: "handle_stats", "Failed to load messages count: {e}");
            return Ok(()); // TODO: send message
        }
    };

    bot.send_message(
        message.chat.id,
        language
            .stats_message
            .replace("{count}", &messages_count.to_string()),
    )
    .await?;
    Ok(())
}
