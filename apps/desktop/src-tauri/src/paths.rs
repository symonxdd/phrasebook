use tauri::command;

#[command(rename_all = "snake_case")]
pub fn get_app_localappdata() -> String {
  phrasebook_core::paths::get_app_localappdata()
}

// #[command(rename_all = "snake_case")]
// pub fn get_app_install_location() -> String {
//   std::env::current_exe()
//     .unwrap()
//     .parent()
//     .unwrap()
//     .to_str()
//     .unwrap()
//     .to_string()
// }
