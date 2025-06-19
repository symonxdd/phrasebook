use crate::db::get_pool;
use crate::models::{EntryEnum, TermTranslation};
use sqlx::Row;

pub async fn get_term_by_id(entry_id: i64, group_id: Option<i64>) -> Result<EntryEnum, String> {
  let pool = get_pool();

  let translations = sqlx::query(
    "SELECT language_code, translation, definition FROM term_translation WHERE term_id = ?",
  )
  .bind(entry_id)
  .fetch_all(pool)
  .await
  .map_err(|e| e.to_string())?
  .into_iter()
  .map(|r| TermTranslation {
    language: r.get("language_code"),
    translation: r.get("translation"),
    definition: r.get("definition"),
  })
  .collect();

  Ok(EntryEnum::Term {
    entry_id,
    group_id,
    translations,
  })
}
