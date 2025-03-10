use dirs::data_local_dir;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Term {
  term: String,
  meaning: String,
  extra: Option<String>,
  category: String,
  tags: Vec<String>,
}

lazy_static::lazy_static! {
  static ref TERMS: Mutex<Vec<Term>> = Mutex::new(Vec::new());
}

fn get_terms_file_path() -> PathBuf {
  let data_dir = data_local_dir().expect("Failed to get local data directory");
  let app_dir = data_dir.join("Phrasebook");
  fs::create_dir_all(&app_dir).expect("Failed to create app data folder");
  app_dir.join("terms.json")
}

#[command(rename_all = "snake_case")]
pub fn load_terms() -> Vec<Term> {
  let file_path = get_terms_file_path();
  let mut file = File::open(&file_path).unwrap_or_else(|_| File::create(&file_path).unwrap());
  let mut data = String::new();
  file.read_to_string(&mut data).unwrap_or(0);

  let terms: Vec<Term> = serde_json::from_str(&data).unwrap_or_else(|_| vec![]);
  let mut terms_lock = TERMS.lock().unwrap();
  *terms_lock = terms.clone();
  terms
}

#[command(rename_all = "snake_case")]
pub fn save_term(term: Term) {
  let mut terms_lock = TERMS.lock().unwrap();
  terms_lock.push(term.clone());

  let json = serde_json::to_string_pretty(&*terms_lock).unwrap();
  let file_path = get_terms_file_path();
  let mut file = File::create(file_path).expect("Failed to write to file");
  file
    .write_all(json.as_bytes())
    .expect("Failed to save data");
}

#[command(rename_all = "snake_case")]
pub fn search_terms(query: String) -> Vec<Term> {
  let terms_lock = TERMS.lock().unwrap();
  let query = query.to_lowercase();

  terms_lock
    .iter()
    .filter(|term| term.term.to_lowercase().contains(&query))
    .cloned()
    .collect()
}
