use crate::db::get_pool;
use serde::Serialize;
use sqlx::Row;

pub async fn get_all_languages() -> Result<Vec<Language>, String> {
  let pool = get_pool();

  let rows = sqlx::query("SELECT code, name FROM language")
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

  Ok(
    rows
      .into_iter()
      .map(|row| Language {
        code: row.get("code"),
        name: row.get("name"),
      })
      .collect(),
  )
}

#[derive(Serialize)]
pub struct Language {
  pub code: String,
  pub name: String,
}
