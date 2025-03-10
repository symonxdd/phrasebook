mod terms;

use terms::{load_terms, save_term, search_terms};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      load_terms,
      save_term,
      search_terms
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
