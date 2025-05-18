//! Represents and implements all possible bot commands.

mod start;
mod stats;

use std::sync::Arc;

use start::handle_start;
use teloxide::{
    Bot, adaptors::DefaultParseMode, prelude::ResponseResult, types::Message,
    utils::command::BotCommands,
};

use crate::modules::i18n::I18nLanguage;

/// Represents a shared set of bot's commands.
#[derive(Debug, BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    /// Represents a `/start` command.
    #[command(description = "Отправляет приветствие пользователю.")]
    Start,
    /// Represents a `/stats` command.
    #[command(description = "Показывает статистику бота в этом чате.")]
    Stats,
}

/// Handles the received command based on its content. See individual
/// functions' documentation for more information.
pub async fn handle_command(
    bot: DefaultParseMode<Bot>,
    message: Message,
    command: Command,
    language: Arc<I18nLanguage>,
) -> ResponseResult<()> {
    match command {
        Command::Start => handle_start(bot, message, language).await,
        Command::Stats => todo!(),
    }
}
