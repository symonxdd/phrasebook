pub mod concept;
pub mod core;
pub mod sentence;
pub mod term;

// Re-export useful structs for convenience
pub use concept::ConceptTitle;
pub use core::{Entry, ExploreResponse};
pub use sentence::SentenceTranslation;
pub use term::TermTranslation;
