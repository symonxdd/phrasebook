#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ConceptTitle {
  pub language: String,
  pub title: String,
}
