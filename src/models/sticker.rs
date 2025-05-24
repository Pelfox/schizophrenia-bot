//! Representation and implementation for `stickers` table.

use std::sync::Arc;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use diesel::{
    ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable,
    dsl::{insert_into, sql},
    sql_types::Float,
};

use crate::{modules::database::PgPool, schema::stickers};

/// Represents a single saved sticker.
#[derive(Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::stickers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Sticker {
    /// Unique ID of this sticker.
    pub id: i64,
    /// When this sticker was created.
    pub created_at: DateTime<Utc>,
    /// ID of the chat that this sticker originated from.
    pub chat_id: i64,
    /// Telegram ID of the sticker.
    pub sticker_id: String,
}

/// Represents data for a new sticker to be saved.
#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::stickers)]
pub struct NewSticker<'a> {
    /// ID of the chat where the sticker was received.
    pub chat_id: i64,
    /// Telegram ID of the sticker for lookup.
    pub sticker_id: &'a str,
}

/// Creates a new `Sticker` in the database with the provided data. Returns an ID
/// of the saved sticker.
pub async fn create_sticker(pool: Arc<PgPool>, sticker: NewSticker<'_>) -> Result<i64> {
    insert_into(stickers::table)
        .values(&sticker)
        .returning(stickers::id)
        .get_result(&mut pool.get()?)
        .context("failed to save the sticker")
}

/// Returns random sticker's ID from a target chat.
pub async fn get_random_sticker_id(pool: Arc<PgPool>, target_chat_id: i64) -> Result<String> {
    use crate::schema::stickers::dsl::*;
    stickers
        .select(sticker_id)
        .filter(chat_id.eq(target_chat_id))
        .order(sql::<Float>("RANDOM()"))
        .first(&mut pool.get()?)
        .context("failed to select a random sticker")
}
