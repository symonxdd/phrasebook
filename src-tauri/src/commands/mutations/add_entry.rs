use crate::db::get_pool;
use sqlx::Row;
use tauri::command;

use crate::models::{ConceptTitle, SentenceTranslation, TermTranslation};

#[command(rename_all = "snake_case")]
pub async fn add_entry(
  entry_type: String,
  group_id: Option<i64>,
  term: Option<Vec<TermTranslation>>,
  concept: Option<(String, Vec<ConceptTitle>)>,
  sentence: Option<Vec<SentenceTranslation>>,
) -> Result<i64, String> {
  let pool = get_pool();

  // 1. Start a database transaction
  let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

  // 2. Insert into entry table and get entry_id
  let entry_id: i64 = sqlx::query("INSERT INTO entry (type, group_id) VALUES (?, ?) RETURNING id")
    .bind(&entry_type)
    .bind(group_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| e.to_string())?
    .get("id");

  // 3. Depending on the entry_type, insert into the corresponding table
  match entry_type.as_str() {
    // 3. Handle case when entry_type is a term
    "term" => {
      sqlx::query("INSERT INTO term (entry_id) VALUES (?)")
        .bind(entry_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

      // Loop through term_translations
      if let Some(term_data) = term {
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
      }
    }

    // 3b. Handle case when entry_type is a sentence
    "sentence" => {
      sqlx::query("INSERT INTO sentence (entry_id) VALUES (?)")
        .bind(entry_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

      if let Some(sentences) = sentence {
        // Loop through sentence_translations
        for s in sentences {
          sqlx::query("INSERT INTO sentence_translation (sentence_id, language_code, sentence) VALUES (?, ?, ?)")
                        .bind(entry_id)
                        .bind(&s.language)
                        .bind(&s.sentence)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;
        }
      }
    }

    // 3c. Handle case when entry_type is a concept
    "concept" => {
      if let Some((markdown, titles)) = concept {
        sqlx::query("INSERT INTO concept (entry_id, markdown_content) VALUES (?, ?)")
          .bind(entry_id)
          .bind(&markdown)
          .execute(&mut *tx)
          .await
          .map_err(|e| e.to_string())?;

        // Loop through concept_titles
        for title in titles {
          sqlx::query(
            "INSERT INTO concept_title (concept_id, language_code, title) VALUES (?, ?, ?)",
          )
          .bind(entry_id)
          .bind(&title.language)
          .bind(&title.title)
          .execute(&mut *tx)
          .await
          .map_err(|e| e.to_string())?;
        }
      }
    }

    _ => return Err("Invalid entry type".into()),
  }

  // Commit the transaction
  tx.commit().await.map_err(|e| e.to_string())?;

  // Return the entry_id
  Ok(entry_id)
}
