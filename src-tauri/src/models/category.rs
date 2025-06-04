use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Category {
  pub name: String,
  #[sqlx(try_from = "String")]
  pub terms_json: Terms,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Terms(pub Vec<String>);

impl TryFrom<String> for Terms {
  type Error = serde_json::Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let vec: Vec<String> = serde_json::from_str(&value)?;
    Ok(Terms(vec))
  }
}
