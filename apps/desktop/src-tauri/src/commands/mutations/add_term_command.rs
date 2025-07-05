use phrasebook_core::commands::mutations::add_term::add_term;
use phrasebook_core::models::TermTranslation;

#[tauri::command(rename_all = "snake_case")]
pub async fn add_term_command(
  group_id: Option<i64>,
  term_data: Vec<TermTranslation>,
) -> Result<i64, String> {
  add_term(group_id, term_data).await
}
