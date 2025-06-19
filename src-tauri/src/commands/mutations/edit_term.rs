use crate::db::get_pool;
use serde::Deserialize;
use sqlx::{query, SqlitePool};

#[tauri::command]
pub async fn edit_term(payload: EditTermPayload) -> Result<(), String> {
  let pool: &'static SqlitePool = get_pool();

  let Some(terms) = &payload.term else {
    return Err("Only 'term' entry_type is supported here".to_string());
  };

  let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

  query("UPDATE entry SET group_id = ? WHERE id = ?")
    .bind(payload.group_id)
    .bind(payload.entry_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

  for term in terms {
    let lang = term.language.trim();
    let translation = term.translation.trim();
    let definition = term.definition.trim();

    let both_empty = translation.is_empty() && definition.is_empty();

    if both_empty {
      query("DELETE FROM term_translation WHERE term_id = ? AND language_code = ?")
        .bind(payload.entry_id) // still use entry_id value as the term_id
        .bind(lang)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    } else {
      query(
        r#"
      INSERT INTO term_translation (term_id, language_code, translation, definition)
      VALUES (?, ?, ?, ?)
      ON CONFLICT(term_id, language_code) DO UPDATE SET
        translation = excluded.translation,
        definition = excluded.definition
      "#,
      )
      .bind(payload.entry_id) // still use entry_id value as the term_id
      .bind(lang)
      .bind(translation)
      .bind(definition)
      .execute(&mut *tx)
      .await
      .map_err(|e| e.to_string())?;
    }
  }

  tx.commit().await.map_err(|e| e.to_string())?;

  Ok(())
}

#[derive(Debug, Deserialize)]
pub struct EditTermPayload {
  entry_id: i64,
  group_id: Option<i64>,
  term: Option<Vec<TermInput>>,
}

#[derive(Debug, Deserialize)]
pub struct TermInput {
  language: String,
  translation: String,
  definition: String,
}
