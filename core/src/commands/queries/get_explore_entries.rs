use crate::db::get_pool;
use crate::models::{ConceptTitle, EntryEnum, SentenceTranslation, TermTranslation};
use sqlx::Row;

pub async fn get_explore_entries(offset: i64, limit: i64) -> Result<Vec<EntryEnum>, String> {
  let pool = get_pool();
  let mut entries: Vec<EntryEnum> = Vec::new();

  // --- Terms ---
  let term_rows = sqlx::query(
    r#"
        SELECT e.id as entry_id, e.group_id,
               json_group_array(json_object(
                 'language', tt.language_code,
                 'translation', tt.translation,
                 'definition', tt.definition
               )) as translations
        FROM entry e
        JOIN term t ON t.entry_id = e.id
        LEFT JOIN term_translation tt ON tt.term_id = t.entry_id
        WHERE e.type = 'term'
        GROUP BY e.id
        LIMIT ? OFFSET ?
        "#,
  )
  .bind(limit)
  .bind(offset)
  .fetch_all(pool)
  .await
  .map_err(|e| e.to_string())?;

  for row in term_rows {
    let translations_json: String = row.get("translations");
    let translations: Vec<TermTranslation> =
      serde_json::from_str(&translations_json).unwrap_or_default();

    entries.push(EntryEnum::Term {
      entry_id: row.get("entry_id"),
      group_id: row.get("group_id"),
      translations,
    });
  }

  // --- Concepts ---
  let concept_rows = sqlx::query(
    r#"
        SELECT e.id as entry_id, e.group_id, c.markdown_content,
               json_group_array(json_object(
                 'language', ct.language_code,
                 'title', ct.title
               )) as titles
        FROM entry e
        JOIN concept c ON c.entry_id = e.id
        LEFT JOIN concept_title ct ON ct.concept_id = c.entry_id
        WHERE e.type = 'concept'
        GROUP BY e.id
        LIMIT ? OFFSET ?
        "#,
  )
  .bind(limit)
  .bind(offset)
  .fetch_all(pool)
  .await
  .map_err(|e| e.to_string())?;

  for row in concept_rows {
    let titles_json: String = row.get("titles");
    let titles: Vec<ConceptTitle> = serde_json::from_str(&titles_json).unwrap_or_default();

    entries.push(EntryEnum::Concept {
      entry_id: row.get("entry_id"),
      group_id: row.get("group_id"),
      markdown_content: row.get("markdown_content"),
      titles,
    });
  }

  // --- Sentences ---
  let sentence_rows = sqlx::query(
    r#"
        SELECT e.id as entry_id, e.group_id,
               json_group_array(json_object(
                 'language', st.language_code,
                 'sentence', st.sentence
               )) as sentences
        FROM entry e
        JOIN sentence s ON s.entry_id = e.id
        LEFT JOIN sentence_translation st ON st.sentence_id = s.entry_id
        WHERE e.type = 'sentence'
        GROUP BY e.id
        LIMIT ? OFFSET ?
        "#,
  )
  .bind(limit)
  .bind(offset)
  .fetch_all(pool)
  .await
  .map_err(|e| e.to_string())?;

  for row in sentence_rows {
    let sentences_json: String = row.get("sentences");
    let sentences: Vec<SentenceTranslation> =
      serde_json::from_str(&sentences_json).unwrap_or_default();

    entries.push(EntryEnum::Sentence {
      entry_id: row.get("entry_id"),
      group_id: row.get("group_id"),
      sentences,
    });
  }

  Ok(entries)
}
