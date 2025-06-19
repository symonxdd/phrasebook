use crate::db::get_pool;
use serde::Deserialize;
use sqlx::{query, SqlitePool};

#[tauri::command]
pub async fn edit_sentence(payload: EditSentencePayload) -> Result<(), String> {
  let pool: &'static SqlitePool = get_pool();

  let Some(sentences) = &payload.sentence else {
    return Err("Only 'sentence' entry_type is supported here".to_string());
  };

  let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

  // Update group_id in entry table
  query("UPDATE entry SET group_id = ? WHERE id = ?")
    .bind(payload.group_id)
    .bind(payload.entry_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

  for sentence in sentences {
    let lang = sentence.language.trim();
    let sentence_text = sentence.sentence.trim();

    if sentence_text.is_empty() {
      // Delete translation if empty
      query("DELETE FROM sentence_translation WHERE sentence_id = ? AND language_code = ?")
        .bind(payload.entry_id)
        .bind(lang)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    } else {
      // Upsert translation
      query(
        r#"
                INSERT INTO sentence_translation (sentence_id, language_code, sentence)
                VALUES (?, ?, ?)
                ON CONFLICT(sentence_id, language_code) DO UPDATE SET
                  sentence = excluded.sentence
                "#,
      )
      .bind(payload.entry_id)
      .bind(lang)
      .bind(sentence_text)
      .execute(&mut *tx)
      .await
      .map_err(|e| e.to_string())?;
    }
  }

  tx.commit().await.map_err(|e| e.to_string())?;
  Ok(())
}

/// Incoming payload for a sentence edit operation
#[derive(Debug, Deserialize)]
pub struct EditSentencePayload {
  pub entry_id: i64,
  pub group_id: Option<i64>,
  pub sentence: Option<Vec<SentenceInput>>,
}

/// One localized sentence + explanation
#[derive(Debug, Deserialize)]
pub struct SentenceInput {
  pub language: String,
  pub sentence: String,
}
