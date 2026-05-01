-- Add migration script here
CREATE TABLE students (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),

    grade_id uuid NOT NULL,

    name TEXT NOT NULL,
    father_name TEXT NOT NULL,

    admission_no INTEGER NOT NULL UNIQUE,

    date_of_birth DATE NOT NULL,

    section_id uuid NULL,

    FOREIGN KEY (grade_id) REFERENCES grades(id),
    FOREIGN KEY (section_id) REFERENCES sections(id)
);

CREATE TABLE sections (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,        
    sort_order INT NOT NULL,   
    UNIQUE (name),
    UNIQUE (sort_order)
);

INSERT INTO sections (name, sort_order) VALUES
  ('A', 0),
  ('B', 1),
  ('C', 2),
  ('D', 3);


