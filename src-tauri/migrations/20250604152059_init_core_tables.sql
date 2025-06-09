-- -------------------------------
-- Core Metadata Tables
-- -------------------------------

-- Table for supported languages
CREATE TABLE language (
    code TEXT PRIMARY KEY, -- e.g., 'en', 'fr', 'de'
    name TEXT NOT NULL
);

-- Table for custom grouping/filtering
CREATE TABLE entry_group (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

-- Central entry table (common metadata)
CREATE TABLE entry (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL CHECK (type IN ('term', 'concept', 'sentence')),
    group_id INTEGER REFERENCES entry_group(id) ON DELETE SET NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_entry_type ON entry(type);
CREATE INDEX idx_entry_group ON entry(group_id);