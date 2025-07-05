use crate::db::get_pool;
use crate::models::{ConceptTitle, EntryEnum};
use sqlx::Row;

pub async fn get_concept_by_id(entry_id: i64, group_id: Option<i64>) -> Result<EntryEnum, String> {
  let pool = get_pool();

  let markdown_content: String =
    sqlx::query("SELECT markdown_content FROM concept WHERE entry_id = ?")
      .bind(entry_id)
      .fetch_one(pool)
      .await
      .map_err(|e| e.to_string())?
      .get("markdown_content");

  let titles = sqlx::query("SELECT language_code, title FROM concept_title WHERE concept_id = ?")
    .bind(entry_id)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?
    .into_iter()
    .map(|r| ConceptTitle {
      language: r.get("language_code"),
      title: r.get("title"),
    })
    .collect();

  Ok(EntryEnum::Concept {
    entry_id,
    group_id,
    markdown_content,
    titles,
  })
}
