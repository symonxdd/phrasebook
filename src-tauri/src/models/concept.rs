// src-tauri/src/models/concept.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConceptTitle {
  pub language: String,
  pub title: String,
}
