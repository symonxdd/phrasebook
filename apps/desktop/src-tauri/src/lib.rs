mod commands; // local Tauri command wrappers
mod paths;

use phrasebook_core::init_db; // imported from the shared `core` crate
use tauri::Manager;
use tauri::async_runtime::block_on;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  block_on(init_db());

  tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_opener::init())
    .setup(|app| {
      let window = app.get_webview_window("main").unwrap();
      let mut title = "Phrasebook".to_string();
      if cfg!(debug_assertions) {
        title.push_str(" (dev)");
      }
      window.set_title(&title)?;
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::queries::get_explore_entries_command::get_explore_entries_command,
      commands::queries::get_all_languages_command::get_all_languages_command,
      commands::queries::get_all_groups_command::get_all_groups_command,
      commands::queries::get_entry_stats_command::get_entry_stats_command,
      commands::queries::get_entry_by_id_command::get_entry_by_id_command,
      commands::queries::search_explore_entries_command::search_explore_entries_command,
      commands::mutations::add_entry_command::add_entry_command,
      commands::mutations::edit_term_command::edit_term_command,
      commands::mutations::edit_sentence_command::edit_sentence_command,
      commands::mutations::edit_concept_command::edit_concept_command,
      commands::mutations::delete_entry_command::delete_entry_command,
      commands::mutations::add_term_command::add_term_command,
      commands::mutations::add_sentence_command::add_sentence_command,
      commands::mutations::add_concept_command::add_concept_command,
      paths::get_app_localappdata
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
