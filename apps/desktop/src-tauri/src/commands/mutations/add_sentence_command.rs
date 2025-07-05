use phrasebook_core::commands::mutations::add_sentence::add_sentence;
use phrasebook_core::models::SentenceTranslation;

#[tauri::command(rename_all = "snake_case")]
pub async fn add_sentence_command(
  group_id: Option<i64>,
  sentence_data: Vec<SentenceTranslation>,
) -> Result<i64, String> {
  add_sentence(group_id, sentence_data).await
}
