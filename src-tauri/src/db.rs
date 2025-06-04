use sqlx::{
  sqlite::{SqliteConnectOptions, SqlitePoolOptions},
  SqlitePool,
};
// Import OnceLock from the standard library for thread-safe single initialization
use crate::paths::get_app_localappdata;
use std::sync::OnceLock;

// Static pool that will be initialized once and reused globally
static DB_POOL: OnceLock<SqlitePool> = OnceLock::new();

/// Initializes the SQLite database connection pool and runs migrations.
/// This is called once at startup.
pub async fn init_db() -> SqlitePool {
  let db_path = get_app_localappdata().replace("\\", "/");
  let db_name = "data.db";

  println!("🗃️  Using SQLite DB at: {}/{}", db_path, db_name);

  let connection = SqliteConnectOptions::new()
    .create_if_missing(true)
    .filename(&format!("{}/{}", db_path, db_name));

  // Establish a connection pool (max 5 connections), creating the DB file if it doesn't exist
  let pool = SqlitePoolOptions::new()
    .max_connections(5)
    .connect_with(connection)
    .await
    .expect("Failed to connect to database");

  // Automatically run migrations from the `migrations/` folder if needed
  sqlx::migrate!("./migrations")
    .run(&pool)
    .await
    .expect("Migrations failed");

  // Store the pool globally so the rest of the app can easily access it
  DB_POOL.set(pool.clone()).unwrap_or_else(|_| ());

  // Return the pool so whoever called init_db can also use it immediately if needed
  pool
}

/// Getter to retrieve the already-initialized global pool anywhere in the app
pub fn get_pool() -> &'static SqlitePool {
  // If init_db wasn't called yet, this will panic — so call init_db at app start
  DB_POOL.get().expect("Database not initialized")
}
