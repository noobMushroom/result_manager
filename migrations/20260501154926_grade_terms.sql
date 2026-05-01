-- Add migration script here
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

