//! Implements database pooling and other database stuff.

use std::env::var;

use anyhow::{Context, Result};
use diesel::{PgConnection, r2d2::ConnectionManager};
use r2d2::Pool;

/// Describes the type for the database pool.
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

/// Initializes the database connection pool.
pub async fn init_pool() -> Result<PgPool> {
    let database_url = var("DATABASE_URL").context("database URL isn't set")?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .context("cannot initialize pool")
}
