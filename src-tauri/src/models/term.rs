use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Term {
  pub term: String,
  pub meaning: String,
  pub extra: Option<String>,

  pub category_id: Option<i32>, // FIXED: INTEGER instead of TEXT
  pub group_id: Option<i32>,    // FIXED: INTEGER instead of TEXT

  // pub category: Option<String>,
  // pub group: Option<String>,
  #[sqlx(try_from = "String")]
  pub tags: Tags,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tags(pub Vec<String>);

impl TryFrom<String> for Tags {
  type Error = serde_json::Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let vec: Vec<String> = serde_json::from_str(&value)?;
    Ok(Tags(vec))
  }
}
