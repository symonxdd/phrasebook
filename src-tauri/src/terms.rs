use crate::models::{Category, Group, Term};
use crate::paths::{get_categories_file_path, get_groups_file_path, get_terms_file_path};
use std::fs::File;
use std::io::{Read, Write};
use std::sync::Mutex;
use tauri::command;

lazy_static::lazy_static! {
  static ref TERMS: Mutex<Vec<Term>> = Mutex::new(Vec::new());
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
pub fn load_groups() -> Vec<Group> {
  let file_path = get_groups_file_path();
  let mut file = File::open(&file_path).unwrap_or_else(|_| File::create(&file_path).unwrap());
  let mut data = String::new();
  file.read_to_string(&mut data).unwrap_or(0);

  serde_json::from_str(&data).unwrap_or_else(|_| vec![])
}

#[command(rename_all = "snake_case")]
pub fn load_categories() -> Vec<Category> {
  let file_path = get_categories_file_path();
  let mut file = File::open(&file_path).unwrap_or_else(|_| File::create(&file_path).unwrap());
  let mut data = String::new();
  file.read_to_string(&mut data).unwrap_or(0);

  serde_json::from_str(&data).unwrap_or_else(|_| vec![])
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
pub fn save_group(group: Group) {
  let mut groups = load_groups();
  groups.push(group.clone());

  let json = serde_json::to_string_pretty(&groups).unwrap();
  let file_path = get_groups_file_path();
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
