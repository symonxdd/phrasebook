// src-tauri/src/models/term.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TermTranslation {
    pub language: String,
    pub translation: Option<String>,
    pub definition: Option<String>,
}
