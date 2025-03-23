use dirs::data_local_dir;
use std::fs;
use std::path::PathBuf;
use tauri::command;

pub fn get_terms_file_path() -> PathBuf {
  let data_dir = data_local_dir().expect("Failed to get local data directory");
  let app_dir = data_dir.join("Phrasebook");
  fs::create_dir_all(&app_dir).expect("Failed to create app data folder");
  app_dir.join("terms.json")
}

pub fn get_groups_file_path() -> PathBuf {
  let data_dir = data_local_dir().expect("Failed to get local data directory");
  let app_dir = data_dir.join("Phrasebook");
  fs::create_dir_all(&app_dir).expect("Failed to create app data folder");
  app_dir.join("groups.json")
}

pub fn get_categories_file_path() -> PathBuf {
  let data_dir = data_local_dir().expect("Failed to get local data directory");
  let app_dir = data_dir.join("Phrasebook");
  fs::create_dir_all(&app_dir).expect("Failed to create app data folder");
  app_dir.join("categories.json")
}

#[command(rename_all = "snake_case")]
pub fn get_app_localappdata() -> String {
  let data_dir = data_local_dir().expect("Failed to get local data directory");
  let app_dir = data_dir.join("Phrasebook");
  fs::create_dir_all(&app_dir).expect("Failed to create app data folder");
  app_dir.to_str().unwrap().to_string()
}
