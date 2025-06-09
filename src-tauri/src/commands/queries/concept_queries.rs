// use crate::db::get_pool;
// use crate::models::concept::{Concept, ConceptTitle};
// use sqlx::FromRow;
// use tauri::command;

// #[derive(FromRow)]
// struct ConceptRow {
//   entry_id: i64,
//   markdown_content: String,
// }

// #[derive(FromRow)]
// struct ConceptTitleRow {
//   id: i64,
//   concept_id: i64,
//   language_code: String,
//   title: String,
// }

// #[command(rename_all = "snake_case")]
// pub async fn get_all_concepts() -> Result<Vec<Concept>, String> {
//   let pool = get_pool();
//   let rows: Vec<ConceptRow> =
//     sqlx::query_as::<_, ConceptRow>("SELECT entry_id, markdown_content FROM concept")
//       .fetch_all(pool)
//       .await
//       .map_err(|e| e.to_string())?;

//   Ok(
//     rows
//       .into_iter()
//       .map(|r| Concept {
//         entry_id: r.entry_id,
//         markdown_content: r.markdown_content,
//       })
//       .collect(),
//   )
// }

// #[command(rename_all = "snake_case")]
// pub async fn get_concept_details(concept_id: i64) -> Result<Vec<ConceptTitle>, String> {
//   let pool = get_pool();
//   let rows: Vec<ConceptTitleRow> = sqlx::query_as(
//     "SELECT id, concept_id, language_code, title FROM concept_title WHERE concept_id = ?",
//   )
//   .bind(concept_id)
//   .fetch_all(pool)
//   .await
//   .map_err(|e| e.to_string())?;

//   Ok(
//     rows
//       .into_iter()
//       .map(|r| ConceptTitle {
//         id: r.id,
//         concept_id: r.concept_id,
//         language_code: r.language_code,
//         title: r.title,
//       })
//       .collect(),
//   )
// }
