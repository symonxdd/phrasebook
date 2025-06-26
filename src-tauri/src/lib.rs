mod commands;
mod db;
mod models;
mod paths;

use db::init_db;
use tauri::async_runtime::block_on;
use tauri::Manager;

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
      commands::queries::get_explore_entries::get_explore_entries,
      commands::queries::common::get_all_languages::get_all_languages,
      commands::queries::common::get_all_groups::get_all_groups,
      commands::queries::common::get_entry_stats::get_entry_stats,
      commands::queries::get_entry_by_id::get_entry_by_id,
      commands::queries::search_explore_entries::search_explore_entries,
      commands::mutations::add_entry::add_entry,
      commands::mutations::edit_term::edit_term,
      commands::mutations::edit_sentence::edit_sentence,
      commands::mutations::edit_concept::edit_concept,
      commands::mutations::delete_entry::delete_entry,
      commands::mutations::add_term::add_term,
      commands::mutations::add_sentence::add_sentence,
      commands::mutations::add_concept::add_concept,
      paths::get_app_localappdata
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
