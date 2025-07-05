use crate::db::get_pool;
use serde::Deserialize;
use sqlx::{SqlitePool, query};

pub async fn delete_entry(payload: DeleteEntryPayload) -> Result<(), String> {
  let pool: &'static SqlitePool = get_pool();

  let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

  // Delete the entry — everything else is handled by cascading and triggers
  query("DELETE FROM entry WHERE id = ?")
    .bind(payload.entry_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

  tx.commit().await.map_err(|e| e.to_string())?;

  Ok(())
}

#[derive(Debug, Deserialize)]
pub struct DeleteEntryPayload {
  pub entry_id: i64,
}
