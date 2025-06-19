use crate::commands::queries::logic::get_concept_by_id::get_concept_by_id;
use crate::commands::queries::logic::get_sentence_by_id::get_sentence_by_id;
use crate::commands::queries::logic::get_term_by_id::get_term_by_id;

use crate::models::EntryEnum;
use sqlx::Row;

#[tauri::command(rename_all = "snake_case")]
pub async fn get_entry_by_id(entry_id: i64) -> Result<EntryEnum, String> {
  let pool = crate::db::get_pool();

  let row = sqlx::query("SELECT * FROM entry WHERE id = ?")
    .bind(entry_id)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

  let entry_type: String = row.get("type");
  let group_id: Option<i64> = row.get("group_id");

  match entry_type.as_str() {
    "term" => get_term_by_id(entry_id, group_id).await,
    "concept" => get_concept_by_id(entry_id, group_id).await,
    "sentence" => get_sentence_by_id(entry_id, group_id).await,
    _ => Err(format!("Unknown entry type: {}", entry_type)),
  }
}
