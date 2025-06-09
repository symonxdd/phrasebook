// use crate::db::get_pool;
// use crate::models::term::{Term, TermTranslation};
// use serde::Serialize;
// use sqlx::FromRow;
// use tauri::command;

// #[derive(FromRow)]
// struct TermRow {
//   entry_id: i64,
// }

// #[derive(FromRow)]
// struct TermTranslationRow {
//   id: i64,
//   term_id: i64,
//   language_code: String,
//   translation: Option<String>,
//   definition: Option<String>,
// }

// #[derive(Serialize)]
// pub struct FullTerm {
//   pub entry_id: i64,
//   pub translations: Vec<TermTranslation>,
// }

// #[command(rename_all = "snake_case")]
// pub async fn get_all_terms() -> Result<Vec<FullTerm>, String> {
//   let pool = get_pool();

//   // Step 1: get all term entry IDs
//   let terms: Vec<TermRow> = sqlx::query_as::<_, TermRow>("SELECT entry_id FROM term")
//     .fetch_all(pool)
//     .await
//     .map_err(|e| e.to_string())?;

//   let mut result = Vec::new();

//   // Step 2: for each term, get its translations
//   for term in terms {
//     let translations: Vec<TermTranslationRow> = sqlx::query_as::<_, TermTranslationRow>(
//       r#"
//     SELECT id, term_id, language_code, translation, definition
//     FROM term_translation
//     WHERE term_id = ?
//     "#,
//     )
//     .bind(term.entry_id)
//     .fetch_all(pool)
//     .await
//     .map_err(|e| e.to_string())?;

//     let mapped = translations
//       .into_iter()
//       .map(|r| TermTranslation {
//         id: r.id,
//         term_id: r.term_id,
//         language_code: r.language_code,
//         translation: r.translation,
//         definition: r.definition,
//       })
//       .collect();

//     result.push(FullTerm {
//       entry_id: term.entry_id,
//       translations: mapped,
//     });
//   }

//   Ok(result)
// }

// #[command(rename_all = "snake_case")]
// pub async fn get_term_details(term_id: i64) -> Result<Vec<TermTranslation>, String> {
//   let pool = get_pool();
//   let rows: Vec<TermTranslationRow> = sqlx::query_as(
//         "SELECT id, term_id, language_code, translation, definition FROM term_translation WHERE term_id = ?"
//     )
//     .bind(term_id)
//     .fetch_all(pool)
//     .await
//     .map_err(|e| e.to_string())?;

//   Ok(
//     rows
//       .into_iter()
//       .map(|r| TermTranslation {
//         id: r.id,
//         term_id: r.term_id,
//         language_code: r.language_code,
//         translation: r.translation,
//         definition: r.definition,
//       })
//       .collect(),
//   )
// }
