use super::concept_title::ConceptTitle;
use super::sentence_translation::SentenceTranslation;
use super::term_translation::TermTranslation;

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum EntryEnum {
  Term {
    entry_id: i64,
    group_id: Option<i64>,
    translations: Vec<TermTranslation>,
  },
  Sentence {
    entry_id: i64,
    group_id: Option<i64>,
    sentences: Vec<SentenceTranslation>,
  },
  Concept {
    entry_id: i64,
    group_id: Option<i64>,
    markdown_content: String,
    titles: Vec<ConceptTitle>,
  },
}
