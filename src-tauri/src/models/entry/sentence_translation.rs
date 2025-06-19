#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SentenceTranslation {
  pub language: String,
  pub sentence: String,
}
