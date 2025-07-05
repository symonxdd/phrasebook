use phrasebook_core::commands::mutations::edit_concept::{EditConceptPayload, edit_concept};

#[tauri::command(rename_all = "snake_case")]
pub async fn edit_concept_command(payload: EditConceptPayload) -> Result<(), String> {
  edit_concept(payload).await
}
