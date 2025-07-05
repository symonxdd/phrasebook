-- -------------------------------
-- Full-Text Search Table (FTS5)
-- -------------------------------

CREATE VIRTUAL TABLE entry_fts USING fts5(
    entry_id UNINDEXED,
    type UNINDEXED,
    language_code UNINDEXED,
    group_id UNINDEXED,
    content, -- aggregated searchable text (translation, definition, title, sentence, etc.)
    tokenize = 'unicode61'
);