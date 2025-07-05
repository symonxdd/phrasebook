use crate::db::get_pool;
use serde::Serialize;
use sqlx::Row;

pub async fn get_entry_stats() -> Result<EntryStats, String> {
  let pool = get_pool();

  let row = sqlx::query(
    r#"
        SELECT
            COUNT(*) AS total,
            SUM(CASE WHEN type = 'term' THEN 1 ELSE 0 END) AS term,
            SUM(CASE WHEN type = 'sentence' THEN 1 ELSE 0 END) AS sentence,
            SUM(CASE WHEN type = 'concept' THEN 1 ELSE 0 END) AS concept
        FROM entry
        "#,
  )
  .fetch_one(pool)
  .await
  .map_err(|e| e.to_string())?;

  Ok(EntryStats {
    total: row.get("total"),
    term: row.get("term"),
    sentence: row.get("sentence"),
    concept: row.get("concept"),
  })
}

#[derive(Serialize)]
pub struct EntryStats {
  total: i64,
  term: i64,
  sentence: i64,
  concept: i64,
}
