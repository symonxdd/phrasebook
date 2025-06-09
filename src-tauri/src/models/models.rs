use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TermTranslation {
  pub language: String,
  pub translation: Option<String>,
  pub definition: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConceptTitle {
  pub language: String,
  pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SentenceTranslation {
  pub language: String,
  pub sentence: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Entry {
  #[serde(rename = "term")]
  Term {
    entry_id: i64,
    group_id: Option<i64>,
    translations: Vec<TermTranslation>,
  },
  #[serde(rename = "concept")]
  Concept {
    entry_id: i64,
    group_id: Option<i64>,
    markdown_content: String,
    titles: Vec<ConceptTitle>,
  },
  #[serde(rename = "sentence")]
  Sentence {
    entry_id: i64,
    group_id: Option<i64>,
    sentences: Vec<SentenceTranslation>,
  },
}

#[derive(Serialize)]
pub struct ExploreResponse {
  pub entries: Vec<Entry>,
}
