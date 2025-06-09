-- -------------------------------
-- Triggers to Populate and Sync FTS
-- -------------------------------

-- Helper trigger for inserting FTS rows for term translations
CREATE TRIGGER trg_insert_term_translation AFTER INSERT ON term_translation
BEGIN
    INSERT INTO entry_fts (entry_id, type, language_code, group_id, content)
    SELECT t.entry_id, 'term', NEW.language_code, e.group_id,
           COALESCE(NEW.translation, '') || ' ' || COALESCE(NEW.definition, '')
    FROM term t
    JOIN entry e ON e.id = t.entry_id
    WHERE t.entry_id = NEW.term_id;
END;

CREATE TRIGGER trg_update_term_translation AFTER UPDATE ON term_translation
BEGIN
    DELETE FROM entry_fts WHERE entry_id = NEW.term_id AND language_code = NEW.language_code;
    INSERT INTO entry_fts (entry_id, type, language_code, group_id, content)
    SELECT t.entry_id, 'term', NEW.language_code, e.group_id,
           COALESCE(NEW.translation, '') || ' ' || COALESCE(NEW.definition, '')
    FROM term t
    JOIN entry e ON e.id = t.entry_id
    WHERE t.entry_id = NEW.term_id;
END;

CREATE TRIGGER trg_delete_term_translation AFTER DELETE ON term_translation
BEGIN
    DELETE FROM entry_fts WHERE entry_id = OLD.term_id AND language_code = OLD.language_code;
END;

-- Triggers for concept titles
CREATE TRIGGER trg_insert_concept_title AFTER INSERT ON concept_title
BEGIN
    INSERT INTO entry_fts (entry_id, type, language_code, group_id, content)
    SELECT c.entry_id, 'concept', NEW.language_code, e.group_id,
           NEW.title || ' ' || c.markdown_content
    FROM concept c
    JOIN entry e ON e.id = c.entry_id
    WHERE c.entry_id = NEW.concept_id;
END;

CREATE TRIGGER trg_update_concept_title AFTER UPDATE ON concept_title
BEGIN
    DELETE FROM entry_fts WHERE entry_id = NEW.concept_id AND language_code = NEW.language_code;
    INSERT INTO entry_fts (entry_id, type, language_code, group_id, content)
    SELECT c.entry_id, 'concept', NEW.language_code, e.group_id,
           NEW.title || ' ' || c.markdown_content
    FROM concept c
    JOIN entry e ON e.id = c.entry_id
    WHERE c.entry_id = NEW.concept_id;
END;

CREATE TRIGGER trg_delete_concept_title AFTER DELETE ON concept_title
BEGIN
    DELETE FROM entry_fts WHERE entry_id = OLD.concept_id AND language_code = OLD.language_code;
END;

-- Also update FTS on concept markdown update
CREATE TRIGGER trg_update_concept_markdown AFTER UPDATE ON concept
BEGIN
    DELETE FROM entry_fts WHERE entry_id = NEW.entry_id;
    INSERT INTO entry_fts (entry_id, type, language_code, group_id, content)
    SELECT NEW.entry_id, 'concept', ct.language_code, e.group_id,
           ct.title || ' ' || NEW.markdown_content
    FROM concept_title ct
    JOIN entry e ON e.id = NEW.entry_id
    WHERE ct.concept_id = NEW.entry_id;
END;

-- Triggers for sentence translations
CREATE TRIGGER trg_insert_sentence_translation AFTER INSERT ON sentence_translation
BEGIN
    INSERT INTO entry_fts (entry_id, type, language_code, group_id, content)
    SELECT s.entry_id, 'sentence', NEW.language_code, e.group_id,
           NEW.sentence
    FROM sentence s
    JOIN entry e ON e.id = s.entry_id
    WHERE s.entry_id = NEW.sentence_id;
END;

CREATE TRIGGER trg_update_sentence_translation AFTER UPDATE ON sentence_translation
BEGIN
    DELETE FROM entry_fts WHERE entry_id = NEW.sentence_id AND language_code = NEW.language_code;
    INSERT INTO entry_fts (entry_id, type, language_code, group_id, content)
    SELECT s.entry_id, 'sentence', NEW.language_code, e.group_id,
           NEW.sentence
    FROM sentence s
    JOIN entry e ON e.id = s.entry_id
    WHERE s.entry_id = NEW.sentence_id;
END;

CREATE TRIGGER trg_delete_sentence_translation AFTER DELETE ON sentence_translation
BEGIN
    DELETE FROM entry_fts WHERE entry_id = OLD.sentence_id AND language_code = OLD.language_code;
END;