use phrasebook_core::commands::mutations::edit_sentence::EditSentencePayload;

#[tauri::command(rename_all = "snake_case")]
pub async fn edit_sentence_command(payload: EditSentencePayload) -> Result<(), String> {
  phrasebook_core::commands::mutations::edit_sentence::edit_sentence(payload).await
}
