use crate::db::get_pool;
use crate::models::{ConceptTitle, Entry, ExploreResponse, SentenceTranslation, TermTranslation};
use sqlx::Row;
use tauri::command;

#[command(rename_all = "snake_case")]
pub async fn search_explore_entries(
  offset: i64,
  limit: i64,
  search: String,
  types: Vec<String>,
  languages: Vec<String>,
) -> Result<ExploreResponse, String> {
  let pool = get_pool();
  let mut entries: Vec<Entry> = Vec::new();

  // 1. Build placeholders like "?, ?, ?, ..."
  let type_placeholders = vec!["?"; types.len()].join(",");
  let lang_placeholders = vec!["?"; languages.len()].join(",");

  // 2. Create query string
  let fts_query = format!(
    r#"
    SELECT DISTINCT entry_id
    FROM entry_fts
    WHERE type IN ({}) AND language_code IN ({}) AND entry_fts MATCH ?
    LIMIT ? OFFSET ?
    "#,
    type_placeholders, lang_placeholders
  );

  // 3. Prepare query and bind parameters
  let mut query = sqlx::query(&fts_query);
  for t in &types {
    query = query.bind(t);
  }
  for l in &languages {
    query = query.bind(l);
  }

  // Add * to the search term
  let formatted_search = format!("{}*", search);
  query = query.bind(&formatted_search).bind(limit).bind(offset);

  // 4. Run query
  let entry_ids: Vec<i64> = query
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?
    .into_iter()
    .map(|row| row.get("entry_id"))
    .collect();

  // 5. Fetch entries one by one
  for id in entry_ids {
    let row = sqlx::query("SELECT * FROM entry WHERE id = ?")
      .bind(id)
      .fetch_one(pool)
      .await
      .map_err(|e| e.to_string())?;
    let entry_type: String = row.get("type");
    let group_id: Option<i64> = row.get("group_id");

    match entry_type.as_str() {
      "term" => {
        let translations = sqlx::query(
          "SELECT language_code, translation, definition FROM term_translation WHERE term_id = ?",
        )
        .bind(id)
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

        entries.push(Entry::Term {
          entry_id: id,
          group_id,
          translations,
        });
      }

      "sentence" => {
        let sentences = sqlx::query(
          "SELECT language_code, sentence FROM sentence_translation WHERE sentence_id = ?",
        )
        .bind(id)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|r| SentenceTranslation {
          language: r.get("language_code"),
          sentence: r.get("sentence"),
        })
        .collect();

        entries.push(Entry::Sentence {
          entry_id: id,
          group_id,
          sentences,
        });
      }

      "concept" => {
        let markdown_content: String =
          sqlx::query("SELECT markdown_content FROM concept WHERE entry_id = ?")
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?
            .get("markdown_content");

        let titles =
          sqlx::query("SELECT language_code, title FROM concept_title WHERE concept_id = ?")
            .bind(id)
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|r| ConceptTitle {
              language: r.get("language_code"),
              title: r.get("title"),
            })
            .collect();

        entries.push(Entry::Concept {
          entry_id: id,
          group_id,
          markdown_content,
          titles,
        });
      }

      _ => {}
    }
  }

  Ok(ExploreResponse { entries })
}
