mod db;
mod models;
mod paths;
mod terms;

use db::init_db;
use tauri::async_runtime::block_on;

use paths::get_app_localappdata;
use terms::{
  get_app_install_location, import_terms_from_text, load_non_empty_categories,
  load_non_empty_groups, load_non_empty_unsorted, load_terms, load_terms_by_category,
  load_terms_by_group, load_terms_by_unsorted, save_group, save_term, search_terms,
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
      save_group,
      get_app_install_location,
      get_app_localappdata,
      load_non_empty_categories,
      load_non_empty_groups,
      load_terms_by_category,
      load_terms_by_group,
      load_terms_by_unsorted,
      import_terms_from_text,
      load_non_empty_unsorted,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
