use phrasebook_core::commands::queries::get_entry_stats::EntryStats;
use phrasebook_core::commands::queries::get_entry_stats::get_entry_stats;

#[tauri::command(rename_all = "snake_case")]
pub async fn get_entry_stats_command() -> Result<EntryStats, String> {
  get_entry_stats().await
}
