//! Contains handlers for all dice-related functionality.

use std::{sync::Arc, time::Duration};

use rand::Rng;
use teloxide::{
    payloads::SendDiceSetters,
    prelude::{Requester, ResponseResult},
    sugar::request::RequestReplyExt,
    types::{Dice, Message},
};
use tokio::time::sleep;

use crate::{bot::Bot, modules::i18n::I18nLanguage};

/// Handles all possible variations of dices.
pub async fn dice_handler(
    bot: Bot,
    message: Message,
    dice: Dice,
    language: Arc<I18nLanguage>,
) -> ResponseResult<()> {
    // answer only with 30% probability
    if !rand::rng().random_bool(0.3) {
        return Ok(());
    }

    bot.send_message(message.chat.id, &language.dice.received)
        .reply_to(message.id)
        .await?;

    let own_message = bot.send_dice(message.chat.id).emoji(dice.emoji).await?;

    if let Some(own_dice) = own_message.dice() {
        if own_dice.value > dice.value {
            sleep(Duration::from_secs(3)).await;
            bot.send_message(message.chat.id, &language.dice.win)
                .await?;
        }
    }

    Ok(())
}
