use crate::db::get_pool;
use crate::models::{EntryEnum, SentenceTranslation};
use sqlx::Row;

pub async fn get_sentence_by_id(entry_id: i64, group_id: Option<i64>) -> Result<EntryEnum, String> {
  let pool = get_pool();

  let sentences =
    sqlx::query("SELECT language_code, sentence FROM sentence_translation WHERE sentence_id = ?")
      .bind(entry_id)
      .fetch_all(pool)
      .await
      .map_err(|e| e.to_string())?
      .into_iter()
      .map(|r| SentenceTranslation {
        language: r.get("language_code"),
        sentence: r.get("sentence"),
      })
      .collect();

  Ok(EntryEnum::Sentence {
    entry_id,
    group_id,
    sentences,
  })
}
