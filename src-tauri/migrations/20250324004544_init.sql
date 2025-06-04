PRAGMA foreign_keys = ON; -- Ensure foreign key support is enabled

CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS terms (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    term TEXT NOT NULL COLLATE NOCASE,
    meaning TEXT,
    extra TEXT,
    category_id INTEGER,  -- FK to categories
    group_id INTEGER,     -- FK to groups
    tags TEXT,            -- JSON string of tags

    FOREIGN KEY (category_id) REFERENCES categories (id) ON DELETE SET NULL,
    FOREIGN KEY (group_id) REFERENCES groups (id) ON DELETE SET NULL
);

-- 🔥 CREATE FTS5 TABLE WITH TRIGRAM TOKENIZER FOR SUBSTRING SEARCH 🔥
CREATE VIRTUAL TABLE IF NOT EXISTS terms_fts USING fts5(
    id UNINDEXED,  -- Store `id` without indexing
    term, 
    meaning, 
    extra, 
    category, 
    "group", 
    tags, 
    content='terms', 
    content_rowid='id',
    tokenize='trigram' -- ✅ Enables substring matching
);

-- 🔥 TRIGGERS TO AUTO-SYNC FTS5 🔥
CREATE TRIGGER IF NOT EXISTS terms_fts_insert AFTER INSERT ON terms
BEGIN
    INSERT INTO terms_fts (rowid, term, meaning, extra, category, "group", tags)
    VALUES (new.id, new.term, new.meaning, new.extra, 
           (SELECT name FROM categories WHERE id = new.category_id),
           (SELECT name FROM groups WHERE id = new.group_id),
           new.tags);
END;

CREATE TRIGGER IF NOT EXISTS terms_fts_update AFTER UPDATE ON terms
BEGIN
    DELETE FROM terms_fts WHERE rowid = old.id;
    INSERT INTO terms_fts (rowid, term, meaning, extra, category, "group", tags)
    VALUES (new.id, new.term, new.meaning, new.extra, 
           (SELECT name FROM categories WHERE id = new.category_id),
           (SELECT name FROM groups WHERE id = new.group_id),
           new.tags);
END;

CREATE TRIGGER IF NOT EXISTS terms_fts_delete AFTER DELETE ON terms
BEGIN
    DELETE FROM terms_fts WHERE rowid = old.id;
END;