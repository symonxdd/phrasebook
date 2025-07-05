#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TermTranslation {
  pub language: String,
  pub translation: String,
  pub definition: String,
}
