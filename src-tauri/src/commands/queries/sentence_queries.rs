// use crate::db::get_pool;
// use crate::models::sentence::{Sentence, SentenceTranslation};
// use sqlx::FromRow;
// use std::collections::HashMap;
// use tauri::command;

// #[derive(FromRow)]
// struct SentenceRow {
//   entry_id: i64,
// }

// #[derive(FromRow)]
// struct SentenceTranslationRow {
//   id: i64,
//   sentence_id: i64,
//   language_code: String,
//   sentence: String,
// }

// #[derive(Debug, FromRow)]
// struct SentenceWithTranslationRow {
//   entry_id: i64,
//   group_id: Option<i64>,
//   created_at: String,
//   updated_at: String,
//   translation_id: Option<i64>,
//   sentence_id: Option<i64>,
//   language_code: Option<String>,
//   sentence: Option<String>,
// }

// #[command(rename_all = "snake_case")]
// pub async fn get_all_sentences() -> Result<Vec<Sentence>, String> {
//   let pool = get_pool();

//   let rows: Vec<SentenceWithTranslationRow> = sqlx::query_as::<_, SentenceWithTranslationRow>(
//     r#"
//         SELECT
//             e.id AS entry_id,
//             e.group_id,
//             e.created_at,
//             e.updated_at,
//             st.id AS translation_id,
//             st.sentence_id,
//             st.language_code,
//             st.sentence
//         FROM sentence s
//         JOIN entry e ON e.id = s.entry_id
//         LEFT JOIN sentence_translation st ON st.sentence_id = s.entry_id
//         WHERE e.type = 'sentence'
//         ORDER BY e.id
//         "#,
//   )
//   .fetch_all(pool)
//   .await
//   .map_err(|e| e.to_string())?;

//   let mut sentence_map: HashMap<i64, Sentence> = HashMap::new();

//   for row in rows {
//     let sentence = sentence_map.entry(row.entry_id).or_insert(Sentence {
//       entry_id: row.entry_id,
//       group_id: row.group_id,
//       created_at: row.created_at.clone(),
//       updated_at: row.updated_at.clone(),
//       translations: vec![],
//     });

//     if let (Some(translation_id), Some(sentence_id), Some(language_code), Some(sentence_text)) = (
//       row.translation_id,
//       row.sentence_id,
//       row.language_code,
//       row.sentence,
//     ) {
//       sentence.translations.push(SentenceTranslation {
//         id: translation_id,
//         sentence_id,
//         language_code,
//         sentence: sentence_text,
//       });
//     }
//   }

//   Ok(sentence_map.into_values().collect())
// }

// #[command(rename_all = "snake_case")]
// pub async fn get_sentence_details(sentence_id: i64) -> Result<Vec<SentenceTranslation>, String> {
//   let pool = get_pool();
//   let rows: Vec<SentenceTranslationRow> = sqlx::query_as(
//         "SELECT id, sentence_id, language_code, sentence FROM sentence_translation WHERE sentence_id = ?"
//     )
//     .bind(sentence_id)
//     .fetch_all(pool)
//     .await
//     .map_err(|e| e.to_string())?;

//   Ok(
//     rows
//       .into_iter()
//       .map(|r| SentenceTranslation {
//         id: r.id,
//         sentence_id: r.sentence_id,
//         language_code: r.language_code,
//         sentence: r.sentence,
//       })
//       .collect(),
//   )
// }
