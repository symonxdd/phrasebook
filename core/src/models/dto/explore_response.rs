use crate::models::EntryEnum;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ExploreResponse {
  pub entries: Vec<EntryEnum>,
}
