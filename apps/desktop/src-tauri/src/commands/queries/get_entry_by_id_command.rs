use phrasebook_core::commands::queries::get_entry_by_id::get_entry_by_id;
use phrasebook_core::models::EntryEnum;

#[tauri::command(rename_all = "snake_case")]
pub async fn get_entry_by_id_command(entry_id: i64) -> Result<EntryEnum, String> {
  get_entry_by_id(entry_id).await
}
