use phrasebook_core::commands::mutations::add_entry::add_entry;
use phrasebook_core::models::ConceptTitle;
use phrasebook_core::models::SentenceTranslation;
use phrasebook_core::models::TermTranslation;

#[tauri::command(rename_all = "snake_case")]
pub async fn add_entry_command(
  entry_type: String,
  group_id: Option<i64>,
  term: Option<Vec<TermTranslation>>,
  concept: Option<(String, Vec<ConceptTitle>)>,
  sentence: Option<Vec<SentenceTranslation>>,
) -> Result<i64, String> {
  add_entry(entry_type, group_id, term, concept, sentence).await
}
