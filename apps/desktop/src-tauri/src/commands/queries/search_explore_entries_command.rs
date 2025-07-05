use phrasebook_core::commands::queries::search_explore_entries::search_explore_entries;
use phrasebook_core::models::EntryEnum;

#[tauri::command(rename_all = "snake_case")]
pub async fn search_explore_entries_command(
  offset: i64,
  limit: i64,
  search: String,
  types: Vec<String>,
  languages: Vec<String>,
) -> Result<Vec<EntryEnum>, String> {
  search_explore_entries(offset, limit, search, types, languages).await
}
