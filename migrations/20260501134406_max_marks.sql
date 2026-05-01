-- Add migration script here
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
