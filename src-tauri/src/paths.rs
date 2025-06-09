use dirs::data_local_dir;
use std::fs;
use tauri::command;

#[command(rename_all = "snake_case")]
pub fn get_app_localappdata() -> String {
  let data_dir = data_local_dir().expect("Failed to get local data directory");
  let app_dir = data_dir.join("Phrasebook");
  fs::create_dir_all(&app_dir).expect("Failed to create app data folder");
  app_dir.to_str().unwrap().to_string()
}

#[command(rename_all = "snake_case")]
pub fn get_app_install_location() -> String {
  std::env::current_exe()
    .unwrap()
    .parent()
    .unwrap()
    .to_str()
    .unwrap()
    .to_string()
}
