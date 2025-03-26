use crate::models::{Category, Group, Term};
use tauri::command;

#[command(rename_all = "snake_case")]
pub fn get_app_install_location() -> String {
  std::env::current_exe()
    .unwrap()
    .parent()
    .unwrap()
    .to_str()
    .unwrap()
    .to_string()
}

#[command(rename_all = "snake_case")]
pub async fn load_terms() -> Vec<Term> {
  let pool = crate::db::get_pool();
  let rows =
    sqlx::query_as::<_, Term>("SELECT term, meaning, extra, category, \"group\", tags FROM terms")
      .fetch_all(pool)
      .await
      .unwrap_or_else(|_| vec![]);
  rows
}

#[command(rename_all = "snake_case")]
pub async fn load_groups() -> Vec<Group> {
  let pool = crate::db::get_pool();
  sqlx::query_as::<_, Group>("SELECT name FROM groups")
    .fetch_all(pool)
    .await
    .unwrap_or_else(|_| vec![])
}

#[command(rename_all = "snake_case")]
pub async fn load_categories() -> Vec<Category> {
  let pool = crate::db::get_pool();
  sqlx::query_as::<_, Category>("SELECT name FROM categories")
    .fetch_all(pool)
    .await
    .unwrap_or_else(|_| vec![])
}

#[command(rename_all = "snake_case")]
pub async fn save_term(term: Term) {
  let pool = crate::db::get_pool();
  let tags_json = serde_json::to_string(&term.tags).unwrap();
  sqlx::query(
    "INSERT INTO terms (term, meaning, extra, category, \"group\", tags) VALUES (?, ?, ?, ?, ?, ?)",
  )
  .bind(term.term)
  .bind(term.meaning)
  .bind(term.extra)
  .bind(term.category)
  .bind(term.group)
  .bind(tags_json)
  .execute(pool)
  .await
  .expect("Failed to save term");
}

#[command(rename_all = "snake_case")]
pub async fn save_group(group: Group) {
  let pool = crate::db::get_pool();
  sqlx::query("INSERT INTO groups (name) VALUES (?)")
    .bind(group.name)
    .execute(pool)
    .await
    .expect("Failed to save group");
}

#[command(rename_all = "snake_case")]
pub async fn search_terms(query: String) -> Vec<Term> {
  let pool = crate::db::get_pool();
  let like_query = format!("%{}%", query.to_lowercase());

  sqlx::query_as::<_, Term>(
    "SELECT term, meaning, extra, category, \"group\", tags FROM terms WHERE LOWER(term) LIKE ?",
  )
  .bind(like_query)
  .fetch_all(pool)
  .await
  .unwrap_or_else(|_| vec![])
}
