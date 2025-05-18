//! Implements the Telegram bot itself with all middlewares, commands, etc.

use std::{env::var, sync::Arc};

use anyhow::{Context, Result};
use teloxide::{
    adaptors::DefaultParseMode,
    dptree,
    prelude::{Dispatcher, RequesterExt},
    types::ParseMode,
};

use crate::{database::PgPool, modules::i18n::I18nLanguage};

/// Describes the type that represents the active bot in the handlers.
pub type Bot = DefaultParseMode<teloxide::Bot>;

/// Initializes the bot and setups commands, middlewares, etc.
pub async fn init_bot(pool: Arc<PgPool>, language: Arc<I18nLanguage>) -> Result<()> {
    let token = var("TELEGRAM_TOKEN").context("Telegram token isn't set")?;
    let bot = teloxide::Bot::new(token).parse_mode(ParseMode::Html);
    let handler = crate::handlers::setup_handlers();

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![pool, language])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}
