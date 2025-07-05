use phrasebook_core::commands::queries::get_all_languages::Language;
use phrasebook_core::commands::queries::get_all_languages::get_all_languages;

#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_languages_command() -> Result<Vec<Language>, String> {
  get_all_languages().await.map_err(|e| e.to_string())
}
