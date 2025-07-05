use crate::db::get_pool;
use serde::Serialize;
use sqlx::Row;

pub async fn get_all_groups() -> Result<Vec<EntryGroup>, String> {
  let pool = get_pool();

  let rows = sqlx::query("SELECT id, name FROM entry_group ORDER BY name")
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

  Ok(
    rows
      .into_iter()
      .map(|row| EntryGroup {
        id: row.get("id"),
        name: row.get("name"),
      })
      .collect(),
  )
}

#[derive(Serialize)]
pub struct EntryGroup {
  pub id: i64,
  pub name: String,
}
