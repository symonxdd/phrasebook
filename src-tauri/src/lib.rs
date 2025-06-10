mod commands;
mod db;
mod models;
mod paths;

use db::init_db;
use tauri::async_runtime::block_on;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  block_on(init_db());

  tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
      commands::queries::dashboard::get_explore_entries,
      commands::queries::common::get_all_languages,
      commands::queries::common::get_entry_stats,
      paths::get_app_localappdata,
      // commands::queries::term_queries::get_all_terms,
      // commands::queries::term_queries::get_term_details,
      // commands::queries::concept_queries::get_concept_details,
      // commands::queries::sentence_queries::get_all_sentences,
      // commands::queries::sentence_queries::get_sentence_details,
      // commands::queries::fts_queries::search_entries,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
