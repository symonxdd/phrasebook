-- Insert sample data into groups table
INSERT INTO groups (name) VALUES ('Technology');
INSERT INTO groups (name) VALUES ('Science');
INSERT INTO groups (name) VALUES ('Arts');

-- Insert sample data into categories table
INSERT INTO categories (name) VALUES ('Programming Languages');
INSERT INTO categories (name) VALUES ('Physics');
INSERT INTO categories (name) VALUES ('Visual Arts');

-- Insert sample data into terms table (with sample tags)
INSERT INTO terms (term, meaning, extra, category, "group", tags) 
VALUES ('Rust', 'A systems programming language focused on safety and performance', 'Compiles to machine code', 'Programming Languages', 'Technology', '["memory safety", "performance"]');

INSERT INTO terms (term, meaning, extra, category, "group", tags) 
VALUES ('Quantum Mechanics', 'The branch of physics that deals with phenomena at microscopic scales', 'Involves the study of particles and waves', 'Physics', 'Science', '["quantum", "physics"]');

INSERT INTO terms (term, meaning, extra, category, "group", tags) 
VALUES ('Impressionism', 'A 19th-century art movement characterized by small, thin brush strokes', 'A response to realism', 'Visual Arts', 'Arts', '["art", "impressionist"]');
