use tauri::command;

#[derive(Debug, serde::Serialize)]
pub struct ImportPreview {
  terms: usize,
  sentences: usize,
  concepts: usize,
}

#[command]
pub fn analyze_import_text(text: String) -> Result<ImportPreview, String> {
  let mut terms = 0;
  let mut sentences = 0;
  let mut concepts = 0;

  for line in text.lines() {
    let lower = line.trim().to_lowercase();

    if lower.starts_with("term:") {
      terms += 1;
    } else if lower.starts_with("sentence:") {
      sentences += 1;
    } else if lower.starts_with("concept:") {
      concepts += 1;
    }
  }

  Ok(ImportPreview {
    terms,
    sentences,
    concepts,
  })
}
