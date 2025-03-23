use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Term {
  pub term: String,
  pub meaning: String,
  pub extra: Option<String>,
  pub category: Option<String>,
  pub group: Option<String>,
  pub tags: Vec<String>,
}
