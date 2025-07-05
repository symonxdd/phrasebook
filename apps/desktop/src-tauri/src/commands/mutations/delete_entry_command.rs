use phrasebook_core::commands::mutations::delete_entry::{DeleteEntryPayload, delete_entry};

#[tauri::command]
pub async fn delete_entry_command(payload: DeleteEntryPayload) -> Result<(), String> {
  delete_entry(payload).await
}
