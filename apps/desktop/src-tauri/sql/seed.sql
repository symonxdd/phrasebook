-- =============================================
-- Sample Data Seeding Script for Multilingual DB
-- =============================================

-- -------------------------------
-- Insert Supported Languages
-- -------------------------------
INSERT INTO language (code, name) VALUES
    ('en', 'English'),
    ('fr', 'French'),
    ('de', 'German'),
    ('es', 'Spanish');

-- -------------------------------
-- Insert Entry Groups
-- -------------------------------
INSERT INTO entry_group (name) VALUES
    ('General Vocabulary'),
    ('Programming Concepts'),
    ('Example Sentences');

-- -------------------------------
-- Insert Term Entries
-- -------------------------------
INSERT INTO entry (type, group_id) VALUES
    ('term', 1), -- ID 1
    ('term', 1); -- ID 2

INSERT INTO term (entry_id) VALUES (1), (2);

-- Term Translations
INSERT INTO term_translation (term_id, language_code, translation, definition) VALUES
    (1, 'en', 'dog', 'A domesticated carnivorous mammal.'),
    (1, 'fr', 'chien', 'Un mammifère carnivore domestiqué.'),
    (1, 'de', 'Hund', 'Ein domestiziertes fleischfressendes Säugetier.'),
    (2, 'en', 'computer', 'An electronic device for storing and processing data.'),
    (2, 'es', 'computadora', 'Un dispositivo electrónico para almacenar y procesar datos.');

-- -------------------------------
-- Insert Concept Entries
-- -------------------------------
INSERT INTO entry (type, group_id) VALUES
    ('concept', 2), -- ID 3
    ('concept', 2); -- ID 4

INSERT INTO concept (entry_id, markdown_content) VALUES
    (3, '### Variable\nA container for storing data values.'),
    (4, '### Function\nA block of code which only runs when it is called.');

-- Concept Titles
INSERT INTO concept_title (concept_id, language_code, title) VALUES
    (3, 'en', 'Variable'),
    (3, 'fr', 'Variable'),
    (4, 'en', 'Function'),
    (4, 'de', 'Funktion');

-- -------------------------------
-- Insert Sentence Entries
-- -------------------------------
INSERT INTO entry (type, group_id) VALUES
    ('sentence', 3), -- ID 5
    ('sentence', 3); -- ID 6

INSERT INTO sentence (entry_id) VALUES (5), (6);

-- Sentence Translations
INSERT INTO sentence_translation (sentence_id, language_code, sentence) VALUES
    (5, 'en', 'The quick brown fox jumps over the lazy dog.'),
    (5, 'fr', 'Le rapide renard brun saute par-dessus le chien paresseux.'),
    (6, 'en', 'Programming is both an art and a science.'),
    (6, 'es', 'La programación es tanto un arte como una ciencia.');

-- Done!
-- Your FTS table will be populated automatically by the triggers.

-- Optional: Test FTS
-- SELECT * FROM entry_fts WHERE content MATCH 'dog';
-- SELECT * FROM entry_fts WHERE content MATCH 'fonction'; -- French title
