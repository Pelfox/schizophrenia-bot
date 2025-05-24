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

/// Pluralizes word based on the provided number and possible forms.
fn pluralize(value: i64, forms: &[String]) -> String {
    let index = if (11..=14).contains(&(value % 100)) {
        2
    } else {
        match value % 10 {
            1 => 0,
            2..=4 => 1,
            _ => 2,
        }
    };

    // TODO: move this formatting out of this
    format!("<b>{}</b> {}", value, forms[index])
}

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

    let value = pluralize(messages_count, &language.stats_message.plurals);
    bot.send_message(
        message.chat.id,
        language.stats_message.base.replace("{value}", &value),
    )
    .await?;
    Ok(())
}
