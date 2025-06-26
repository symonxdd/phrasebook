use crate::db::get_pool;
use crate::models::SentenceTranslation;
use sqlx::Row;
use tauri::command;

#[command(rename_all = "snake_case")]
pub async fn add_sentence(
    group_id: Option<i64>,
    sentence_data: Vec<SentenceTranslation>,
) -> Result<i64, String> {
    let pool = get_pool();
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let entry_id: i64 = sqlx::query("INSERT INTO entry (type, group_id) VALUES ('sentence', ?) RETURNING id")
        .bind(group_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?
        .get("id");

    sqlx::query("INSERT INTO sentence (entry_id) VALUES (?)")
        .bind(entry_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    for s in sentence_data {
        sqlx::query("INSERT INTO sentence_translation (sentence_id, language_code, sentence) VALUES (?, ?, ?)")
            .bind(entry_id)
            .bind(&s.language)
            .bind(&s.sentence)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
    }

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(entry_id)
}
