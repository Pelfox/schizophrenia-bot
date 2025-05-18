//! Representation and implementation for `messages` table.

use std::sync::Arc;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use diesel::{
    ExpressionMethods, Insertable, Selectable,
    dsl::{insert_into, sql},
    prelude::*,
    sql_types::Float,
};

use crate::{database::PgPool, schema::messages};

/// Represents a single saved message.
#[derive(Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    /// Unique ID of this message.
    pub id: i64,
    /// When this message was created.
    pub created_at: DateTime<Utc>,
    /// ID of the chat that this message originated from.
    pub chat_id: i64,
    /// Safe content of this message.
    pub content: String,
    /// ID of the user that sent the message.
    pub author_id: i64,
}

/// Represents data for a new message to be saved.
#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::messages)]
pub struct NewMessage<'a> {
    /// ID of the chat where the message was received.
    pub chat_id: i64,
    /// Save content of the message.
    pub content: &'a str,
    /// ID of the message author.
    pub author_id: i64,
}

/// Creates a new `Message` in the database, which holds the content of the
/// received message by a bot. Returns an ID of the created message.
pub async fn create_message(pool: Arc<PgPool>, message: NewMessage<'_>) -> Result<i64> {
    insert_into(messages::table)
        .values(&message)
        .returning(messages::id)
        .get_result(&mut pool.get()?)
        .context("failed to save the message")
}

/// Returns a set of random messages contents for a target chat ID.
pub async fn get_random_messages_content(
    pool: Arc<PgPool>,
    target_chat_id: i64,
) -> Result<Vec<String>> {
    use crate::schema::messages::dsl::*;
    messages
        .select(content)
        .filter(chat_id.eq(target_chat_id))
        .order(sql::<Float>("RANDOM()"))
        .load(&mut pool.get()?)
        .context("failed to randomly select messages")
}
