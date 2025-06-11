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
      commands::queries::dashboard::get_explore_entries,
      commands::queries::common::get_all_languages,
      commands::queries::common::get_entry_stats,
      paths::get_app_localappdata
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
