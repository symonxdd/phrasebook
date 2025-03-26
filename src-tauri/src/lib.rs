mod db;
mod models;
mod paths;
mod terms;

use db::init_db;
use tauri::async_runtime::block_on;

use paths::get_app_localappdata;
use terms::{
  get_app_install_location, load_categories, load_groups, load_terms, save_group, save_term,
  search_terms,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  block_on(init_db());

  tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
      load_terms,
      save_term,
      search_terms,
      load_categories,
      load_groups,
      save_group,
      get_app_install_location,
      get_app_localappdata
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
