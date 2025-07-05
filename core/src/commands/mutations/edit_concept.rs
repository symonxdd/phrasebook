use crate::db::get_pool;
use serde::Deserialize;
use sqlx::{SqlitePool, query};

pub async fn edit_concept(payload: EditConceptPayload) -> Result<(), String> {
  let pool: &'static SqlitePool = get_pool();

  let Some(concept) = &payload.concept else {
    return Err("Only 'concept' entry_type is supported here".to_string());
  };

  let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

  query("UPDATE entry SET group_id = ? WHERE id = ?")
    .bind(payload.group_id)
    .bind(payload.entry_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

  query("UPDATE concept SET markdown_content = ? WHERE entry_id = ?")
    .bind(concept.markdown_content.trim())
    .bind(payload.entry_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

  for title in &concept.titles {
    let lang = title.language.trim();
    let title_text = title.title.trim();

    if title_text.is_empty() {
      query("DELETE FROM concept_title WHERE concept_id = ? AND language_code = ?")
        .bind(payload.entry_id)
        .bind(lang)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    } else {
      query(
        r#"
                INSERT INTO concept_title (concept_id, language_code, title)
                VALUES (?, ?, ?)
                ON CONFLICT(concept_id, language_code) DO UPDATE SET
                    title = excluded.title
                "#,
      )
      .bind(payload.entry_id)
      .bind(lang)
      .bind(title_text)
      .execute(&mut *tx)
      .await
      .map_err(|e| e.to_string())?;
    }
  }

  tx.commit().await.map_err(|e| e.to_string())?;
  Ok(())
}

#[derive(Debug, Deserialize)]
pub struct EditConceptPayload {
  pub entry_id: i64,
  pub group_id: Option<i64>,
  pub concept: Option<ConceptInput>,
}

#[derive(Debug, Deserialize)]
pub struct ConceptInput {
  pub markdown_content: String,
  pub titles: Vec<ConceptTitleInput>,
}

#[derive(Debug, Deserialize)]
pub struct ConceptTitleInput {
  pub language: String,
  pub title: String,
}
