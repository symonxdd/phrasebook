use crate::models::{term::Tags, Category, Group, Term};
use regex::Regex;
use tauri::command;

#[command(rename_all = "snake_case")]
pub async fn load_terms() -> Vec<Term> {
  let pool = crate::db::get_pool();

  let result = sqlx::query_as::<_, Term>(
    r#"
        SELECT 
            t.term, 
            t.meaning, 
            t.extra, 
            t.category_id, 
            t.group_id, 
            t.tags
        FROM terms t
        "#,
  )
  .fetch_all(pool)
  .await;

  match result {
    Ok(rows) => rows,
    Err(err) => {
      println!("SQL Query Failed: {}", err);
      vec![]
    }
  }
}

#[command(rename_all = "snake_case")]
pub async fn search_terms(query: String) -> Vec<Term> {
  let pool = crate::db::get_pool();
  let match_query = format!("{}*", query.to_lowercase()); // Prefix search

  let result = sqlx::query_as::<_, Term>(
    "SELECT terms.id, terms.term, terms.meaning, terms.extra, 
                terms.category_id AS category_id, 
                terms.group_id AS group_id, 
                terms.tags
         FROM terms_fts
         JOIN terms ON terms_fts.rowid = terms.id
         WHERE terms_fts.term MATCH ?",
  )
  .bind(match_query)
  .fetch_all(pool)
  .await;

  match result {
    Ok(terms) => {
      println!("Found {} terms", terms.len());
      for term in &terms {
        println!("Found term: {:?}", term);
      }
      terms
    }
    Err(err) => {
      println!("SQL Query Failed: {}", err);
      vec![]
    }
  }
}

#[command(rename_all = "snake_case")]
pub async fn load_non_empty_categories() -> Vec<Category> {
  let pool = crate::db::get_pool();
  let query = r#"
        SELECT c.id, c.name, 
        json_group_array(t.term) AS terms_json
        FROM categories c
        JOIN (
            SELECT term, category_id
            FROM terms
            WHERE category_id IS NOT NULL
            GROUP BY category_id, term
            ORDER BY category_id, term
            LIMIT 5
        ) t ON c.id = t.category_id
        GROUP BY c.id, c.name;
    "#;

  let rows = sqlx::query_as::<_, Category>(query)
    .fetch_all(pool)
    .await
    .unwrap_or_else(|_| vec![]);

  rows
}

#[command(rename_all = "snake_case")]
pub async fn load_non_empty_groups() -> Vec<Group> {
  let pool = crate::db::get_pool();
  let query = r#"
       SELECT g.id, g.name, 
       json_group_array(t.term) AS terms_json
        FROM groups g
        JOIN (
            SELECT term, group_id
            FROM terms
            WHERE group_id IS NOT NULL
            GROUP BY group_id, term
            ORDER BY group_id, term
            LIMIT 5
        ) t ON g.id = t.group_id
        GROUP BY g.id, g.name;
    "#;

  let rows = sqlx::query_as::<_, Group>(query)
    .fetch_all(pool)
    .await
    .unwrap_or_else(|_| vec![]);

  rows
}

#[command(rename_all = "snake_case")]
pub async fn load_non_empty_unsorted() -> Vec<Term> {
  let pool = crate::db::get_pool();
  let query = r#"
        SELECT t.id, t.term, t.meaning, t.extra, t.category_id, t.group_id, t.tags
        FROM terms t
        WHERE t.group_id IS NULL AND t.category_id IS NULL
        ORDER BY t.term
        LIMIT 5;
    "#;

  let rows = sqlx::query_as::<_, Term>(query)
    .fetch_all(pool)
    .await
    .unwrap_or_else(|_| vec![]);

  rows
}

#[command(rename_all = "snake_case")]
pub async fn load_terms_by_category(name: String) -> Vec<Term> {
  let pool = crate::db::get_pool();

  let result = sqlx::query_as::<_, Term>(
    "SELECT t.* FROM terms t 
     INNER JOIN categories c ON t.category_id = c.id
     WHERE c.name = ?",
  )
  .bind(name)
  .fetch_all(pool)
  .await;

  match result {
    Ok(rows) => rows,
    Err(_) => vec![],
  }
}

#[command(rename_all = "snake_case")]
pub async fn load_terms_by_group(name: String) -> Vec<Term> {
  let pool = crate::db::get_pool();

  let result = sqlx::query_as::<_, Term>(
    "SELECT t.* FROM terms t 
     INNER JOIN groups g ON t.group_id = g.id
     WHERE g.name = ?",
  )
  .bind(name)
  .fetch_all(pool)
  .await;

  match result {
    Ok(rows) => rows,
    Err(_) => vec![],
  }
}

#[command(rename_all = "snake_case")]
pub async fn load_terms_by_unsorted() -> Vec<Term> {
  let pool = crate::db::get_pool();
  let query = r#"
        SELECT t.id, t.term, t.meaning, t.extra, t.category_id, t.group_id, t.tags
        FROM terms t
        WHERE t.group_id IS NULL AND t.category_id IS NULL
        ORDER BY t.term;
    "#;

  let rows = sqlx::query_as::<_, Term>(query)
    .fetch_all(pool)
    .await
    .unwrap_or_else(|_| vec![]);

  rows
}

#[command(rename_all = "snake_case")]
pub async fn save_term(term: Term) {
  let pool = crate::db::get_pool();
  let tags_json = serde_json::to_string(&term.tags).unwrap();

  // Remove leading "- " if it exists
  let cleaned_term = term.term.strip_prefix("- ").unwrap_or(&term.term);

  let mut tx = pool.begin().await.unwrap();

  sqlx::query(
    "INSERT INTO terms (term, meaning, extra, category_id, group_id, tags) 
         VALUES (?, ?, ?, ?, ?, ?)",
  )
  .bind(cleaned_term) // 🔥 Save cleaned term
  .bind(&term.meaning)
  .bind(&term.extra)
  .bind(&term.category_id) // ✅ Use category_id instead of name
  .bind(&term.group_id) // ✅ Use group_id instead of name
  .bind(&tags_json)
  .execute(&mut *tx)
  .await
  .expect("Failed to save term");

  tx.commit().await.unwrap();
}

#[command(rename_all = "snake_case")]
pub async fn save_group(group: Group) {
  let pool = crate::db::get_pool();
  sqlx::query("INSERT INTO groups (name) VALUES (?)")
    .bind(group.name)
    .execute(pool)
    .await
    .expect("Failed to save group");
}

#[command(rename_all = "snake_case")]
pub fn get_app_install_location() -> String {
  std::env::current_exe()
    .unwrap()
    .parent()
    .unwrap()
    .to_str()
    .unwrap()
    .to_string()
}

#[command(rename_all = "snake_case")]
pub async fn import_terms_from_text(notes: String) -> String {
  let pool = crate::db::get_pool();
  let terms = parse_markdown_notes(&notes);

  if terms.is_empty() {
    return "No terms found in the input.".to_string();
  }

  let mut tx = pool.begin().await.expect("Failed to start transaction");

  for term in &terms {
    let result = sqlx::query(
      "INSERT INTO terms (term, meaning, extra, category_id, group_id, tags) 
             VALUES (?, ?, ?, NULL, NULL, '[]')",
    )
    .bind(&term.term)
    .bind(&term.meaning)
    .bind(&term.extra)
    .execute(&mut *tx)
    .await;

    if let Err(err) = result {
      println!("Failed to insert term '{}': {}", term.term, err);
    }
  }

  tx.commit().await.expect("Failed to commit transaction");

  format!("Imported {} terms.", terms.len())
}

pub fn parse_markdown_notes(input: &str) -> Vec<Term> {
  let mut terms = Vec::new();
  let re = Regex::new(r"^(.+?)[=:—-]\s*(.+)$").unwrap(); // Match "term = meaning" or "term - meaning"

  for line in input.lines() {
    if let Some(captures) = re.captures(line) {
      let term = captures.get(1).unwrap().as_str().trim().to_string();
      let meaning = captures.get(2).unwrap().as_str().trim().to_string();

      terms.push(Term {
        term,
        meaning,
        extra: None,
        category_id: None,
        group_id: None,
        tags: Tags(vec![]), // Default empty tags
      });
    }
  }

  terms
}

// #[command(rename_all = "snake_case")]
// pub async fn load_groups() -> Vec<Group> {
//   let pool = crate::db::get_pool();
//   sqlx::query_as::<_, Group>("SELECT name FROM groups")
//     .fetch_all(pool)
//     .await
//     .unwrap_or_else(|_| vec![])
// }

// #[command(rename_all = "snake_case")]
// pub async fn load_categories() -> Vec<Category> {
//   let pool = crate::db::get_pool();
//   sqlx::query_as::<_, Category>("SELECT name FROM categories")
//     .fetch_all(pool)
//     .await
//     .unwrap_or_else(|_| vec![])
// }
