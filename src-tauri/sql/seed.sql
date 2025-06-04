-- Ensure foreign key support is enabled
PRAGMA foreign_keys = ON;

-- Insert sample data into groups table
INSERT INTO groups (name) VALUES ('Technology') ON CONFLICT(name) DO NOTHING;
INSERT INTO groups (name) VALUES ('Science') ON CONFLICT(name) DO NOTHING;
INSERT INTO groups (name) VALUES ('Arts') ON CONFLICT(name) DO NOTHING;

-- Insert sample data into categories table
INSERT INTO categories (name) VALUES ('Programming Languages') ON CONFLICT(name) DO NOTHING;
INSERT INTO categories (name) VALUES ('Physics') ON CONFLICT(name) DO NOTHING;
INSERT INTO categories (name) VALUES ('Visual Arts') ON CONFLICT(name) DO NOTHING;

-- Insert sample data into terms table
INSERT INTO terms (term, meaning, extra, category_id, group_id, tags) 
VALUES  
    ('Rust', 'A systems programming language focused on safety and performance', 'Compiles to machine code', 
    (SELECT id FROM categories WHERE name = 'Programming Languages'), 
    (SELECT id FROM groups WHERE name = 'Technology'), 
    '["memory safety", "performance"]'),

    ('Quantum Mechanics', 'The branch of physics that deals with phenomena at microscopic scales', 'Involves the study of particles and waves', 
    (SELECT id FROM categories WHERE name = 'Physics'), 
    (SELECT id FROM groups WHERE name = 'Science'), 
    '["quantum", "physics"]'),

    ('Impressionism', 'A 19th-century art movement characterized by small, thin brush strokes', 'A response to realism', 
    (SELECT id FROM categories WHERE name = 'Visual Arts'), 
    (SELECT id FROM groups WHERE name = 'Arts'), 
    '["art", "impressionist"]');

INSERT INTO terms (term, meaning, extra, category_id, group_id, tags) 
VALUES 
    ('Serendipity', 'The occurrence of events by chance in a happy or beneficial way', 'Often used in scientific discoveries', NULL, NULL, '["luck", "chance"]'),
    
    ('Petrichor', 'The pleasant, earthy smell after fresh rain', 'Caused by oils released from the soil', NULL, NULL, '["smell", "rain"]'),

    ('Sonder', 'The realization that each passerby has a life as vivid and complex as your own', 'A term from The Dictionary of Obscure Sorrows', NULL, NULL, '["philosophy", "emotion"]');
