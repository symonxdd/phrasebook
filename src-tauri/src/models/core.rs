// src-tauri/src/models/core.rs
use serde::{Serialize, Deserialize};
use super::term::TermTranslation;
use super::concept::ConceptTitle;
use super::sentence::SentenceTranslation;

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
    }
}

#[derive(Serialize)]
pub struct ExploreResponse {
    pub entries: Vec<Entry>,
}
