use crate::db::get_pool;
use crate::models::TermTranslation;
use sqlx::Row;

pub async fn add_term(
  group_id: Option<i64>,
  term_data: Vec<TermTranslation>,
) -> Result<i64, String> {
  let pool = get_pool();
  let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

  let entry_id: i64 =
    sqlx::query("INSERT INTO entry (type, group_id) VALUES ('term', ?) RETURNING id")
      .bind(group_id)
      .fetch_one(&mut *tx)
      .await
      .map_err(|e| e.to_string())?
      .get("id");

  sqlx::query("INSERT INTO term (entry_id) VALUES (?)")
    .bind(entry_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

  for t in term_data {
    sqlx::query("INSERT INTO term_translation (term_id, language_code, translation, definition) VALUES (?, ?, ?, ?)")
            .bind(entry_id)
            .bind(&t.language)
            .bind(&t.translation)
            .bind(&t.definition)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
  }

  tx.commit().await.map_err(|e| e.to_string())?;
  Ok(entry_id)
}
