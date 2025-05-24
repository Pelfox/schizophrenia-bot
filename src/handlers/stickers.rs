//! Contains handlers for all stickers-related functionality.

use std::sync::Arc;

use log::{error, info};
use teloxide::{
    prelude::ResponseResult,
    types::{Message, Sticker},
};

use crate::{
    bot::Bot,
    models::sticker::{NewSticker, create_sticker},
    modules::database::PgPool,
};

/// Handles all incoming stickers (saves them).
pub async fn stickers_handler(
    _: Bot,
    message: Message,
    sticker: Sticker,
    pool: Arc<PgPool>,
) -> ResponseResult<()> {
    let new_sticker = NewSticker {
        chat_id: message.chat.id.0,
        sticker_id: &sticker.file.id,
    };

    match create_sticker(pool, new_sticker).await {
        Ok(_) => {
            info!(target: "stickers_handler", "Saved sticker from {}: {}", message.chat.id, sticker.file.id);
        }
        Err(e) => {
            error!(target: "stickers_handler", "Failed to save the sticker: {e}");
            return Ok(());
        }
    }

    Ok(())
}
