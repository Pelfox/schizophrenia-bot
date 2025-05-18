//! An internationalization module for all text contents in the bot.
use anyhow::Result;
use std::env::var;

use anyhow::Context;
use log::info;
use serde::Deserialize;
use tokio::fs::read;

/// Holder for all dice-related messages.
#[derive(Debug, Deserialize)]
pub struct LanguageDice {
    /// Sent when user have sent a dice and chance has returned `true`.
    pub received: String,
    /// Sent when the bot have won the user.
    pub win: String,
}

/// Generic holder for all i18n contents.
#[derive(Debug, Deserialize)]
pub struct I18nLanguage {
    /// Sent as a reply to user's `/start` command.
    pub start_message: String,
    /// Sent when bot is added to the group.
    pub greeting_message: String,
    /// See `LanguageDice` for more information.
    pub dice: LanguageDice,
    /// Sent when `/stats` command is executed.
    pub stats_message: String,
}

/// Loads a single language from the provided by env (or default) path.
pub async fn load_language() -> Result<I18nLanguage> {
    let path = var("LANGUAGE_PATH").unwrap_or_else(|_| "language.json".to_owned());
    info!(target: "i18n", "Loading language from {path}");

    let file = read(path).await.context("failed to read language file")?;
    serde_json::from_slice(&file).context("failed to parse JSON")
}
