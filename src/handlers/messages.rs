//! Contains handlers for all messages-related functionality.

use std::{ops::Range, sync::Arc};

use log::error;
use markov::Chain;
use rand::Rng;
use teloxide::{
    prelude::{Requester, ResponseResult},
    types::{ChatAction, Message},
};
use tokio::join;

use crate::{
    bot::Bot,
    database::PgPool,
    models::message::{NewMessage, create_message, get_random_messages_content},
};

/// Minimum amount of messages required for generator to work properly.
const MINIMUM_MESSAGES_SIZE: usize = 50;

/// Order for a generator to create message based on.
const MARKOV_MESSAGES_ORDER: usize = 2;

/// Minimum and maximum amount of sentences being taken into final result.
const SENTENCES_RANGE: Range<usize> = 5..30;

/// Probability in percents that bot will start a Markov interaction.
const PERCENT_PROBABILITY: f64 = 5.0;

/// Handler for all messages: in parallel tries to reply to a received message
/// and saves the new message.
pub async fn save_and_answer(bot: Bot, message: Message, pool: Arc<PgPool>) -> ResponseResult<()> {
    // accept messages only from groups
    if !message.chat.is_group() && !message.chat.is_supergroup() {
        return Ok(());
    }

    let (result_1, result_2) = join!(
        try_answer(bot, &message, pool.clone()),
        save_message(&message, pool)
    );

    // propagate the first error encountered, if any
    result_1?;
    result_2?;

    Ok(())
}

/// Tries to save the received message to the database to train on it later.
async fn save_message(message: &Message, pool: Arc<PgPool>) -> ResponseResult<()> {
    #[allow(deprecated)] // fix clippy warning for a time
    let (content, author) = match (message.text(), message.from()) {
        (Some(text), Some(author)) => (text, author),
        _ => return Ok(()),
    };

    let chat_id = message.chat.id.0;
    let author_id = author.id.0 as i64;
    let new_message = NewMessage {
        content,
        chat_id,
        author_id,
    };

    if let Err(e) = create_message(pool, new_message).await {
        error!(target:"save_message", "Failed to save the message from {chat_id}: {e}");
    };

    Ok(())
}

/// Tries to answer to a message with a specific probability.
async fn try_answer(bot: Bot, message: &Message, pool: Arc<PgPool>) -> ResponseResult<()> {
    // answer only with 20% probability
    if !rand::rng().random_bool(PERCENT_PROBABILITY / 100.0) {
        return Ok(());
    }

    let messages = match get_random_messages_content(pool, message.chat.id.0).await {
        Ok(messages) => messages,
        Err(e) => {
            error!(target: "try_answer", "Failed to retrieve messages for a generator: {e}");
            return Ok(());
        }
    };

    // set fixed amount of messages as a minimum
    if messages.len() <= MINIMUM_MESSAGES_SIZE {
        return Ok(());
    }

    // send "is typing..." action to show that bot is working since generation
    // can take some time
    bot.send_chat_action(message.chat.id, ChatAction::Typing)
        .await?;

    let mut chain = Chain::of_order(MARKOV_MESSAGES_ORDER);
    chain.feed(messages);

    let markov_text = chain.generate_str();
    let sentences_count = rand::rng().random_range(SENTENCES_RANGE);
    let result = markov_text
        .split_whitespace()
        .take(sentences_count)
        .collect::<Vec<&str>>()
        .join(" ");

    bot.send_message(message.chat.id, result).await?;
    Ok(())
}
