// src-tauri/src/models/sentence.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SentenceTranslation {
  pub language: String,
  pub sentence: String,
}
