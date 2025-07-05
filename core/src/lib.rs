// This makes all core logic portable, testable, and available to both Flutter and Tauri.

pub mod commands;
pub mod db;
pub mod models;
pub mod paths;

pub use db::{get_pool, init_db};
