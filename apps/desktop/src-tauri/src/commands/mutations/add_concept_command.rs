use phrasebook_core::commands::mutations::add_concept::add_concept;
use phrasebook_core::models::ConceptTitle;

#[tauri::command(rename_all = "snake_case")]
pub async fn add_concept_command(
  group_id: Option<i64>,
  markdown: String,
  titles: Vec<ConceptTitle>,
) -> Result<i64, String> {
  add_concept(group_id, markdown, titles).await
}
