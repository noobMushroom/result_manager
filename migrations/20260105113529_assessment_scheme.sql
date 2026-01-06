-- Add migration script here
CREATE TABLE assessment_scheme (
    grade_id uuid NOT NULL,
    exam_id uuid NOT NULL,
    subject_id uuid NOT NULL,

    evaluation_type TEXT NOT NULL,  -- 'MARKS' or 'GRADE'
    max_marks INT,                  -- NULL if GRADE

    PRIMARY KEY (grade_id, exam_id, subject_id),

    CHECK (
        (evaluation_type = 'MARKS' AND max_marks IS NOT NULL)
        OR
        (evaluation_type = 'GRADE' AND max_marks IS NULL)
    )
);

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 30
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'Nursery'
  AND et.code = 'WRITT'
  AND t.name IN ('Half Yearly Term', 'Second Term', 'Annual Term')
  AND s.code IN ('MATHS', 'ENG', 'HIN');

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'Nursery'
  AND et.code = 'HW'
  AND t.name IN ('Half Yearly Term', 'Second Term', 'Annual Term')
  AND s.code IN ('MATHS', 'ENG', 'HIN');

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'Nursery'
  AND et.code = 'CN'
  AND t.name IN ('Half Yearly Term', 'Second Term', 'Annual Term')
  AND s.code IN ('MATHS', 'ENG', 'HIN');

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'Nursery'
  AND et.code = 'ORAL_PRAC'
  AND t.name IN ('Half Yearly Term', 'Second Term', 'Annual Term')
  AND s.code IN ('ENG_ORAL', 'ENG_RHYMES','HIN_ORAL', 'HIN_RHYMES','MATHS_ORAL');

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'Nursery'
  AND et.code = 'ACTIVITY'
  AND t.name IN ('Half Yearly Term', 'Second Term', 'Annual Term')
  AND s.code IN ('WELL_DRESSED', 'DRAW','PT', 'WORKSHEET');

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'Nursery'
  AND et.code = 'CLASS_PER'
  AND t.name IN ('Half Yearly Term', 'Second Term', 'Annual Term')
  AND s.code IN ('CLASS_PERF');
