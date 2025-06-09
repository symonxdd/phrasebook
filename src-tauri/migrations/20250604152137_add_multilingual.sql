-- -------------------------------
-- Multilingual Content Tables
-- -------------------------------

-- TERM translations and definitions
CREATE TABLE term_translation (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    term_id INTEGER NOT NULL REFERENCES term(entry_id) ON DELETE CASCADE,
    language_code TEXT NOT NULL REFERENCES language(code),
    translation TEXT,
    definition TEXT,
    UNIQUE(term_id, language_code)
);

-- CONCEPT multilingual titles
CREATE TABLE concept_title (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    concept_id INTEGER NOT NULL REFERENCES concept(entry_id) ON DELETE CASCADE,
    language_code TEXT NOT NULL REFERENCES language(code),
    title TEXT NOT NULL,
    UNIQUE(concept_id, language_code)
);

-- SENTENCE multilingual content
CREATE TABLE sentence_translation (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sentence_id INTEGER NOT NULL REFERENCES sentence(entry_id) ON DELETE CASCADE,
    language_code TEXT NOT NULL REFERENCES language(code),
    sentence TEXT NOT NULL,
    UNIQUE(sentence_id, language_code)
);

-- Indexes for filtering and lookups
CREATE INDEX idx_term_translation_lang ON term_translation(language_code);
CREATE INDEX idx_concept_title_lang ON concept_title(language_code);
CREATE INDEX idx_sentence_translation_lang ON sentence_translation(language_code);