-- -------------------------------
-- Type-Specific Tables
-- -------------------------------

-- TERM: has multilingual definitions and translations
CREATE TABLE term (
    entry_id INTEGER PRIMARY KEY REFERENCES entry(id) ON DELETE CASCADE
    -- additional term-specific fields can go here
);

-- CONCEPT: has multilingual titles and markdown content
CREATE TABLE concept (
    entry_id INTEGER PRIMARY KEY REFERENCES entry(id) ON DELETE CASCADE,
    markdown_content TEXT NOT NULL -- assumed language-independent
);

-- SENTENCE: multilingual sentence content
CREATE TABLE sentence (
    entry_id INTEGER PRIMARY KEY REFERENCES entry(id) ON DELETE CASCADE
    -- additional sentence-specific fields can go here
);