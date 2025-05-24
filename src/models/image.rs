//! Representation and implementation for `images` table.

use std::sync::Arc;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, RunQueryDsl, Selectable, dsl::insert_into};

use crate::{modules::database::PgPool, schema::images};

/// Represents a single saved image.
#[derive(Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::images)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Image {
    /// Unique ID of this image.
    pub id: i64,
    /// When this image was created.
    pub created_at: DateTime<Utc>,
    /// ID of the chat that this image originated from.
    pub chat_id: i64,
    /// Internal ID of the image for lookup.
    pub image_id: String,
}

/// Represents data for a new image to be saved.
#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::images)]
pub struct NewImage<'a> {
    /// ID of the chat where the image was received.
    pub chat_id: i64,
    /// Internal ID of the image for lookup.
    pub image_id: &'a str,
}

/// Creates a new `Image` in the database with the provided data. Returns an ID
/// of the saved image.
pub async fn create_image(pool: Arc<PgPool>, image: NewImage<'_>) -> Result<i64> {
    insert_into(images::table)
        .values(&image)
        .returning(images::id)
        .get_result(&mut pool.get()?)
        .context("failed to save the image")
}
