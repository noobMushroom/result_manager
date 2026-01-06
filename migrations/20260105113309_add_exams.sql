-- Add migration script here

CREATE TABLE terms (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    sort_order INT NOT NULL,
    UNIQUE (name),
    UNIQUE (sort_order)
);

INSERT INTO terms (name, sort_order) VALUES
    ('First Term', 1),
    ('Second Term', 2),
    ('Annual Term', 3),
    ('Half Yearly Term', 4);

CREATE TABLE exam_types (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    UNIQUE (name),
    UNIQUE (code)
);

INSERT INTO exam_types (name, code) VALUES
  ('Unit Test', 'UT'),
  ('Class Performance', 'CLASS_PER'),
  ('Copy Work', 'COPY_WORK'),
  ('Class Work', 'CLASS_WORK'),
  ('Activity', 'ACTIVITY'),
  ('Half Yearly', 'HALF_YEARLY'),
  ('Oral/Practical', 'ORAL_PRAC'),
  ('Written', 'WRITT'),
  ('Copy Neatness', 'CN'),
  ('Homework', 'HW'),
  ('Annual', 'ANNUAL');


CREATE TABLE exams (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    exam_type_id uuid NOT NULL,
    term_id uuid NOT NULL,
    display_name TEXT NOT NULL,

    UNIQUE (exam_type_id, term_id),

    FOREIGN KEY (exam_type_id) REFERENCES exam_types(id),
    FOREIGN KEY (term_id) REFERENCES terms(id)
);


-- UT-I
INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Unit Test I'
FROM exam_types et, terms t
WHERE et.code = 'UT' AND t.name = 'First Term';

-- UT-II
INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Unit Test II'
FROM exam_types et, terms t
WHERE et.code = 'UT' AND t.name = 'Second Term';

-- Half Yearly
INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Half Yearly'
FROM exam_types et, terms t
WHERE et.code = 'HALF_YEARLY' AND t.name = 'First Term';

-- Annual
INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Annual'
FROM exam_types et, terms t
WHERE et.code = 'ANNUAL' AND t.name = 'Second Term';

--  Copy Work
INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Copy Work'
FROM exam_types et, terms t
WHERE et.code = 'COPY_WORK' AND t.name = 'First Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Copy Work'
FROM exam_types et, terms t
WHERE et.code = 'COPY_WORK' AND t.name = 'Half Yearly Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Copy Work'
FROM exam_types et, terms t
WHERE et.code = 'COPY_WORK' AND t.name = 'Second Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Copy Work'
FROM exam_types et, terms t
WHERE et.code = 'COPY_WORK' AND t.name = 'Annual Term';


--  Class Work
INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Class Work'
FROM exam_types et, terms t
WHERE et.code = 'CLASS_WORK' AND t.name = 'First Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Class Work'
FROM exam_types et, terms t
WHERE et.code = 'CLASS_WORK' AND t.name = 'Half Yearly Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Class Work'
FROM exam_types et, terms t
WHERE et.code = 'CLASS_WORK' AND t.name = 'Second Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Class Work'
FROM exam_types et, terms t
WHERE et.code = 'CLASS_WORK' AND t.name = 'Annual Term';

-- Class performance
INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Class Per'
FROM exam_types et, terms t
WHERE et.code = 'CLASS_PER' AND t.name = 'First Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Class Per'
FROM exam_types et, terms t
WHERE et.code = 'CLASS_PER' AND t.name = 'Half Yearly Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Copy Work'
FROM exam_types et, terms t
WHERE et.code = 'CLASS_PER' AND t.name = 'Second Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Class Per'
FROM exam_types et, terms t
WHERE et.code = 'CLASS_PER' AND t.name = 'Annual Term';

-- Oral/Practical
INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Oral/Prac'
FROM exam_types et, terms t
WHERE et.code = 'ORAL_PRAC' AND t.name = 'First Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Oral/Prac'
FROM exam_types et, terms t
WHERE et.code = 'ORAL_PRAC' AND t.name = 'Half Yearly Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Oral/Prac'
FROM exam_types et, terms t
WHERE et.code = 'ORAL_PRAC' AND t.name = 'Second Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Oral/Prac'
FROM exam_types et, terms t
WHERE et.code = 'ORAL_PRAC' AND t.name = 'Annual Term';

-- Written
INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Writt'
FROM exam_types et, terms t
WHERE et.code = 'WRITT' AND t.name = 'First Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Writt'
FROM exam_types et, terms t
WHERE et.code = 'WRITT' AND t.name = 'Half Yearly Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Writt'
FROM exam_types et, terms t
WHERE et.code = 'WRITT' AND t.name = 'Second Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Writt'
FROM exam_types et, terms t
WHERE et.code = 'WRITT' AND t.name = 'Annual Term';

-- Homework 
INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Homework'
FROM exam_types et, terms t
WHERE et.code = 'HW' AND t.name = 'First Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Homework'
FROM exam_types et, terms t
WHERE et.code = 'HW' AND t.name = 'Half Yearly Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Homework'
FROM exam_types et, terms t
WHERE et.code = 'HW' AND t.name = 'Second Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Homework'
FROM exam_types et, terms t
WHERE et.code = 'HW' AND t.name = 'Annual Term';


-- Copy Neatness
INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Copy Neatness'
FROM exam_types et, terms t
WHERE et.code = 'CN' AND t.name = 'First Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Copy Neatness'
FROM exam_types et, terms t
WHERE et.code = 'CN' AND t.name = 'Half Yearly Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Copy Neatness'
FROM exam_types et, terms t
WHERE et.code = 'CN' AND t.name = 'Second Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Copy Neatness'
FROM exam_types et, terms t
WHERE et.code = 'CN' AND t.name = 'Annual Term';

-- Activity
INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Activity'
FROM exam_types et, terms t
WHERE et.code = 'ACTIVITY' AND t.name = 'First Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Activity'
FROM exam_types et, terms t
WHERE et.code = 'ACTIVITY' AND t.name = 'Half Yearly Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Activity'
FROM exam_types et, terms t
WHERE et.code = 'ACTIVITY' AND t.name = 'Second Term';

INSERT INTO exams (exam_type_id, term_id, display_name)
SELECT et.id, t.id, 'Activity'
FROM exam_types et, terms t
WHERE et.code = 'ACTIVITY' AND t.name = 'Annual Term';

