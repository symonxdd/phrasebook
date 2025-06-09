// use crate::db::get_pool;
// use sqlx::FromRow;
// use tauri::command;

// #[derive(Debug, serde::Serialize, FromRow)]
// pub struct SearchResult {
//   pub entry_id: i64,
//   pub r#type: String,
//   pub language_code: String,
//   pub group_id: Option<i64>,
//   pub content: String,
// }

// #[command(rename_all = "snake_case")]
// pub async fn search_entries(
//   query: String,
//   language_code: Option<String>,
// ) -> Result<Vec<SearchResult>, String> {
//   let pool = get_pool();

//   let sql = match language_code {
//     Some(_) => {
//       "SELECT entry_id, type, language_code, group_id, content
//              FROM entry_fts
//              WHERE content MATCH ? AND language_code = ?"
//     }
//     None => {
//       "SELECT entry_id, type, language_code, group_id, content
//              FROM entry_fts
//              WHERE content MATCH ?"
//     }
//   };

//   let rows = match language_code {
//     Some(lang) => {
//       sqlx::query_as::<_, SearchResult>(sql)
//         .bind(query)
//         .bind(lang)
//         .fetch_all(pool)
//         .await
//     }
//     None => {
//       sqlx::query_as::<_, SearchResult>(sql)
//         .bind(query)
//         .fetch_all(pool)
//         .await
//     }
//   };

//   rows.map_err(|e| e.to_string())
// }
