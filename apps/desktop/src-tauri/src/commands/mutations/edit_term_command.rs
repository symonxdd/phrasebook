use phrasebook_core::commands::mutations::edit_term::EditTermPayload;

#[tauri::command(rename_all = "snake_case")]
pub async fn edit_term_command(payload: EditTermPayload) -> Result<(), String> {
  phrasebook_core::commands::mutations::edit_term::edit_term(payload).await
}
