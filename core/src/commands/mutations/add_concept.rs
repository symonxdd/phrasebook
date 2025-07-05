use crate::db::get_pool;
use crate::models::ConceptTitle;
use sqlx::Row;

pub async fn add_concept(
  group_id: Option<i64>,
  markdown: String,
  titles: Vec<ConceptTitle>,
) -> Result<i64, String> {
  let pool = get_pool();
  let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

  let entry_id: i64 =
    sqlx::query("INSERT INTO entry (type, group_id) VALUES ('concept', ?) RETURNING id")
      .bind(group_id)
      .fetch_one(&mut *tx)
      .await
      .map_err(|e| e.to_string())?
      .get("id");

  sqlx::query("INSERT INTO concept (entry_id, markdown_content) VALUES (?, ?)")
    .bind(entry_id)
    .bind(&markdown)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

  for title in titles {
    sqlx::query("INSERT INTO concept_title (concept_id, language_code, title) VALUES (?, ?, ?)")
      .bind(entry_id)
      .bind(&title.language)
      .bind(&title.title)
      .execute(&mut *tx)
      .await
      .map_err(|e| e.to_string())?;
  }

  tx.commit().await.map_err(|e| e.to_string())?;
  Ok(entry_id)
}
