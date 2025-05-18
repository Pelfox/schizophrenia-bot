#![deny(missing_debug_implementations, missing_docs)]

//! A simple Telegram bot for my friends :D.

use log::info;
use modules::{database, i18n::load_language};
use std::sync::Arc;

pub mod modules;

pub mod bot;
pub mod commands;
pub mod handlers;
pub mod models;

mod schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();
    info!(target: "bootstrap", "Starting bot, v{}", env!("CARGO_PKG_VERSION"));

    let language = Arc::new(load_language().await?);
    let pool = Arc::new(database::init_pool().await?);

    bot::init_bot(pool, language).await
}
