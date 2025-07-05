use phrasebook_core::commands::queries::get_all_groups::EntryGroup;
use phrasebook_core::commands::queries::get_all_groups::get_all_groups;

#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_groups_command() -> Result<Vec<EntryGroup>, String> {
  get_all_groups().await
}
