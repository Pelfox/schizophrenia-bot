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

/// Probability in percents that bot will start a dice interaction.
const PERCENT_PROBABILITY: f64 = 30.0;

use crate::{bot::Bot, modules::i18n::I18nLanguage};

/// Handles all possible variations of dices.
pub async fn dice_handler(
    bot: Bot,
    message: Message,
    dice: Dice,
    language: Arc<I18nLanguage>,
) -> ResponseResult<()> {
    // answer only with 20% probability
    if !rand::rng().random_bool(PERCENT_PROBABILITY / 100.0) {
        return Ok(());
    }

    let dice_received_message = bot
        .send_message(message.chat.id, &language.dice.received)
        .reply_to(message.id)
        .await?;

    let own_message = bot
        .send_dice(message.chat.id)
        .emoji(dice.emoji)
        .reply_to(dice_received_message.id)
        .await?;

    if let Some(own_dice) = own_message.dice() {
        if own_dice.value > dice.value {
            sleep(Duration::from_secs(2)).await;
            bot.send_message(message.chat.id, &language.dice.win)
                .reply_to(message.id)
                .await?;
        }
    }

    Ok(())
}
