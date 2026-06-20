-- Add migration script here
-- This table defines which class has which subjects
CREATE TABLE grade_subjects (
  grade_id uuid,
  subject_id uuid,

  PRIMARY KEY (grade_id, subject_id),

  FOREIGN KEY (grade_id) REFERENCES grades(id),
  FOREIGN KEY (subject_id) REFERENCES subjects(id)
);

INSERT INTO grade_subjects (grade_id, subject_id)
SELECT g.id, s.id
FROM grades g
JOIN subjects s ON TRUE
WHERE g.name = 'NURSERY'
  AND s.code IN ('ENG', 'HIN', 'MATHS', 'ENG_ORAL', 'ENG_RHYMES','HIN_ORAL', 'HIN_RHYMES','MATHS_ORAL')
  ON CONFLICT DO NOTHING;

INSERT INTO grade_subjects (grade_id, subject_id)
SELECT g.id, s.id
FROM grades g
JOIN subjects s ON TRUE
WHERE g.name = 'LKG'
  AND s.code IN ('ENG', 'HIN', 'MATHS', 'ENG_ORAL', 'ENG_RHYMES','HIN_ORAL', 'HIN_RHYMES','MATHS_ORAL')
  ON CONFLICT DO NOTHING;

INSERT INTO grade_subjects (grade_id, subject_id)
SELECT g.id, s.id
FROM grades g
JOIN subjects s ON TRUE
WHERE g.name = 'UKG'
  AND s.code IN ('ENG', 'HIN', 'MATHS', 'GS', 'ENG_ORAL', 'ENG_RHYMES', 'HIN_ORAL', 'HIN_RHYMES','MATHS_ORAL', 'GS_ORAL')
  ON CONFLICT DO NOTHING;

INSERT INTO grade_subjects (grade_id, subject_id)
SELECT g.id, s.id
FROM grades g
JOIN subjects s ON TRUE
WHERE g.name = '1'
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'EVS', 'COMP', 'GK', 'DRAW', 'PT')
  ON CONFLICT DO NOTHING;

INSERT INTO grade_subjects (grade_id, subject_id)
SELECT g.id, s.id
FROM grades g
JOIN subjects s ON TRUE
WHERE g.name = '2'
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'EVS', 'COMP', 'GK', 'DRAW', 'PT')
  ON CONFLICT DO NOTHING;

INSERT INTO grade_subjects (grade_id, subject_id)
SELECT g.id, s.id
FROM grades g
JOIN subjects s ON TRUE
WHERE g.name = '3'
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'EVS', 'SS', 'COMP', 'GK', 'DRAW', 'PT')
  ON CONFLICT DO NOTHING;

INSERT INTO grade_subjects (grade_id, subject_id)
SELECT g.id, s.id
FROM grades g
JOIN subjects s ON TRUE
WHERE g.name = '4'
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'EVS', 'SS', 'COMP', 'GK', 'DRAW', 'PT')
  ON CONFLICT DO NOTHING;

INSERT INTO grade_subjects (grade_id, subject_id)
SELECT g.id, s.id
FROM grades g
JOIN subjects s ON TRUE
WHERE g.name = '5'
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'EVS', 'SS', 'COMP', 'GK', 'DRAW', 'PT')
  ON CONFLICT DO NOTHING;

INSERT INTO grade_subjects (grade_id, subject_id)
SELECT g.id, s.id
FROM grades g
JOIN subjects s ON TRUE
WHERE g.name = '6'
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI', 'SS', 'COMP', 'GK', 'DRAW', 'PT')
  ON CONFLICT DO NOTHING;

INSERT INTO grade_subjects (grade_id, subject_id)
SELECT g.id, s.id
FROM grades g
JOIN subjects s ON TRUE
WHERE g.name = '7'
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI', 'SS', 'COMP', 'GK', 'DRAW', 'PT')
  ON CONFLICT DO NOTHING;

INSERT INTO grade_subjects (grade_id, subject_id)
SELECT g.id, s.id
FROM grades g
JOIN subjects s ON TRUE
WHERE g.name = '8'
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI', 'SS', 'COMP', 'GK', 'DRAW', 'PT')
  ON CONFLICT DO NOTHING;

-- This table defines which grade has which exam in a certain term  example nursery half-yearly term exams writt, hw, cn
CREATE TABLE grade_terms (
  grade_id uuid NOT NULL,
  exam_type_id uuid NOT NULL,
  term_id uuid NOT NULL,

  PRIMARY KEY (grade_id, exam_type_id, term_id),

  FOREIGN KEY (grade_id) REFERENCES grades(id),
  FOREIGN KEY (exam_type_id) REFERENCES exam_types(id),
  FOREIGN KEY (term_id) REFERENCES terms(id)
);

-- NURSERY
INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = 'NURSERY'
  AND t.name = 'Half Yearly Term'
  AND et.code IN ('WRITT', 'HW', 'CN');

INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = 'NURSERY'
  AND t.name = 'Second Term'
  AND et.code IN ('WRITT', 'HW', 'CN');

INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = 'NURSERY'
  AND t.name = 'Annual Term'
  AND et.code IN ('WRITT', 'HW', 'CN');

-- LKG
INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = 'LKG'
  AND t.name = 'First Term'
  AND et.code IN ('WRITT', 'HW', 'CN');

INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = 'LKG'
  AND t.name = 'Half Yearly Term'
  AND et.code IN ('WRITT', 'HW', 'CN');

INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = 'LKG'
  AND t.name = 'Second Term'
  AND et.code IN ('WRITT', 'HW', 'CN');

INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = 'LKG'
  AND t.name = 'Annual Term'
  AND et.code IN ('WRITT', 'HW', 'CN');

-- UKG
INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = 'UKG'
  AND t.name = 'First Term'
  AND et.code IN ('WRITT', 'HW', 'CN');

INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = 'UKG'
  AND t.name = 'Half Yearly Term'
  AND et.code IN ('WRITT', 'HW', 'CN');

INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = 'UKG'
  AND t.name = 'Second Term'
  AND et.code IN ('WRITT', 'HW', 'CN');

INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = 'UKG'
  AND t.name = 'Annual Term'
  AND et.code IN ('WRITT', 'HW', 'CN');

-- 1
INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = '1'
  AND t.name = 'First Term'
  AND et.code IN ('UT-I', 'ORAL_PRAC', 'HALF_YEARLY');

INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = '1'
  AND t.name = 'Second Term'
  AND et.code IN ('UT-II', 'ORAL_PRAC', 'ANNUAL');

-- 2
INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = '2'
  AND t.name = 'First Term'
  AND et.code IN ('UT-I', 'ORAL_PRAC', 'HALF_YEARLY');

INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = '2'
  AND t.name = 'Second Term'
  AND et.code IN ('UT-II', 'ORAL_PRAC', 'ANNUAL');

-- 3
INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = '3'
  AND t.name = 'First Term'
  AND et.code IN ('UT-I', 'HALF_YEARLY');

INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = '3'
  AND t.name = 'Second Term'
  AND et.code IN ('UT-II', 'ANNUAL');

-- 4
INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = '4'
  AND t.name = 'First Term'
  AND et.code IN ('UT-I', 'HALF_YEARLY');

INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = '4'
  AND t.name = 'Second Term'
  AND et.code IN ('UT-II', 'ANNUAL');

-- 5
INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = '5'
  AND t.name = 'First Term'
  AND et.code IN ('UT-I', 'HALF_YEARLY');

INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = '5'
  AND t.name = 'Second Term'
  AND et.code IN ('UT-II', 'ANNUAL');

-- 6
INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = '6'
  AND t.name = 'First Term'
  AND et.code IN ('UT-I', 'HALF_YEARLY');

INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = '6'
  AND t.name = 'Second Term'
  AND et.code IN ('UT-II', 'ANNUAL');

-- 7
INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = '7'
  AND t.name = 'First Term'
  AND et.code IN ('UT-I', 'HALF_YEARLY');

INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = '7'
  AND t.name = 'Second Term'
  AND et.code IN ('UT-II', 'ANNUAL');

-- 8
INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = '8'
  AND t.name = 'First Term'
  AND et.code IN ('UT-I', 'HALF_YEARLY');

INSERT INTO grade_terms (grade_id, exam_type_id, term_id)
SELECT g.id, et.id, t.id
FROM grades g, exam_types et, terms t
WHERE g.name = '8'
  AND t.name = 'Second Term'
  AND et.code IN ('UT-II', 'ANNUAL');

-- This table define max marks for each exam for different classes example: - grade 1, exam type ut-1 ut-2 max marks 60
CREATE TABLE max_marks (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),

  grade_id uuid NOT NULL,
  exam_type_id uuid NOT NULL,
  max_marks integer NOT NULL,

  UNIQUE (grade_id, exam_type_id),

  FOREIGN KEY (grade_id) REFERENCES grades(id),
  FOREIGN KEY (exam_type_id) REFERENCES exam_types(id)
);

-- NURSERY
INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 30
FROM grades g
JOIN exam_types et 
  ON et.code = 'WRITT'
WHERE g.name = 'NURSERY'
ON CONFLICT DO NOTHING;

INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 10
FROM grades g
JOIN exam_types et 
  ON et.code = 'HW'
WHERE g.name = 'NURSERY'
ON CONFLICT DO NOTHING;

INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 10
FROM grades g
JOIN exam_types et 
  ON et.code = 'CN'
WHERE g.name = 'NURSERY'
ON CONFLICT DO NOTHING;


-- LKG 
INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 30
FROM grades g
JOIN exam_types et 
  ON et.code = 'WRITT'
WHERE g.name = 'LKG'
ON CONFLICT DO NOTHING;

INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 10
FROM grades g
JOIN exam_types et 
  ON et.code = 'HW'
WHERE g.name = 'LKG'
ON CONFLICT DO NOTHING;

INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 10
FROM grades g
JOIN exam_types et 
  ON et.code = 'CN'
WHERE g.name = 'LKG'
ON CONFLICT DO NOTHING;

-- UKG 
INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 50
FROM grades g
JOIN exam_types et 
  ON et.code = 'WRITT'
WHERE g.name = 'UKG'
ON CONFLICT DO NOTHING;

INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 10
FROM grades g
JOIN exam_types et 
  ON et.code = 'HW'
WHERE g.name = 'UKG'
ON CONFLICT DO NOTHING;

INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 10
FROM grades g
JOIN exam_types et 
  ON et.code = 'CN'
WHERE g.name = 'UKG'
ON CONFLICT DO NOTHING;


-- 1 
INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 20
FROM grades g
JOIN exam_types et 
  ON et.code IN ('UT-I', 'UT-II')
WHERE g.name = '1'
ON CONFLICT DO NOTHING;

INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 20
FROM grades g
JOIN exam_types et 
  ON et.code = 'ORAL_PRAC'
WHERE g.name = '1'
ON CONFLICT DO NOTHING;

INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 60
FROM grades g
JOIN exam_types et 
  ON et.code IN ('HALF_YEARLY', 'ANNUAL')
WHERE g.name = '1'
ON CONFLICT DO NOTHING;

-- 2 
INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 20
FROM grades g
JOIN exam_types et 
  ON et.code IN ('UT-I', 'UT-II')
WHERE g.name = '2'
ON CONFLICT DO NOTHING;

INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 20
FROM grades g
JOIN exam_types et 
  ON et.code = 'ORAL_PRAC'
WHERE g.name = '2'
ON CONFLICT DO NOTHING;

INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 60
FROM grades g
JOIN exam_types et 
  ON et.code IN ('HALF_YEARLY', 'ANNUAL')
WHERE g.name = '2'
ON CONFLICT DO NOTHING;

-- 3 
INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 30
FROM grades g
JOIN exam_types et 
  ON et.code IN ('UT-I', 'UT-II')
WHERE g.name = '3'
ON CONFLICT DO NOTHING;

INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 70
FROM grades g
JOIN exam_types et 
  ON et.code IN ('HALF_YEARLY', 'ANNUAL')
WHERE g.name = '3'
ON CONFLICT DO NOTHING;

-- 4 
INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 30
FROM grades g
JOIN exam_types et 
  ON et.code IN ('UT-I', 'UT-II')
WHERE g.name = '4'
ON CONFLICT DO NOTHING;

INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 70
FROM grades g
JOIN exam_types et 
  ON et.code IN ('HALF_YEARLY', 'ANNUAL')
WHERE g.name = '4'
ON CONFLICT DO NOTHING;

-- 5 
INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 30
FROM grades g
JOIN exam_types et 
  ON et.code IN ('UT-I', 'UT-II')
WHERE g.name = '5'
ON CONFLICT DO NOTHING;

INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 70
FROM grades g
JOIN exam_types et 
  ON et.code IN ('HALF_YEARLY', 'ANNUAL')
WHERE g.name = '5'
ON CONFLICT DO NOTHING;

-- 6 
INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 30
FROM grades g
JOIN exam_types et 
  ON et.code IN ('UT-I', 'UT-II')
WHERE g.name = '6'
ON CONFLICT DO NOTHING;

INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 70
FROM grades g
JOIN exam_types et 
  ON et.code IN ('HALF_YEARLY', 'ANNUAL')
WHERE g.name = '6'
ON CONFLICT DO NOTHING;

-- 7 
INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 30
FROM grades g
JOIN exam_types et 
  ON et.code IN ('UT-I', 'UT-II')
WHERE g.name = '7'
ON CONFLICT DO NOTHING;

INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 70
FROM grades g
JOIN exam_types et 
  ON et.code IN ('HALF_YEARLY', 'ANNUAL')
WHERE g.name = '7'
ON CONFLICT DO NOTHING;

-- 8 
INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 30
FROM grades g
JOIN exam_types et 
  ON et.code IN ('UT-I', 'UT-II')
WHERE g.name = '8'
ON CONFLICT DO NOTHING;

INSERT INTO max_marks (grade_id, exam_type_id, max_marks)
SELECT g.id, et.id, 70
FROM grades g
JOIN exam_types et 
  ON et.code IN ('HALF_YEARLY', 'ANNUAL')
WHERE g.name = '8'
ON CONFLICT DO NOTHING;
