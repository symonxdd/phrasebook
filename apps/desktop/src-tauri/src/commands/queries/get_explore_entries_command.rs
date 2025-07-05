use phrasebook_core::commands::queries::get_explore_entries::get_explore_entries;
use phrasebook_core::models::EntryEnum;

#[tauri::command(rename_all = "snake_case")]
pub async fn get_explore_entries_command(
  offset: i64,
  limit: i64,
) -> Result<Vec<EntryEnum>, String> {
  get_explore_entries(offset, limit).await
}
