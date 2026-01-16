-- Add migration script here
CREATE TABLE assessment_scheme (
    id UUID DEFAULT uuid_generate_v4(),
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
WHERE g.name = 'NURSERY'
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
WHERE g.name = 'NURSERY'
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
WHERE g.name = 'NURSERY'
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
WHERE g.name = 'NURSERY'
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
WHERE g.name = 'NURSERY'
  AND et.code = 'ACTIVITY'
  AND t.name IN ('Half Yearly Term', 'Annual Term')
  AND s.code IN ('WELL_DRESSED', 'DRAW','PT', 'WORKSHEET');

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'NURSERY'
  AND et.code = 'ACTIVITY'
  AND t.name IN ( 'Second Term')
  AND s.code IN ('WELL_DRESSED', 'DRAW','PT');

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'NURSERY'
  AND et.code = 'CLASS_PER'
  AND t.name IN ('Half Yearly Term', 'Second Term', 'Annual Term')
  AND s.code IN ('CLASS_PERF');

-- LKG

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 30
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'LKG'
  AND et.code = 'WRITT'
  AND t.name IN ('First Term', 'Half Yearly Term', 'Second Term', 'Annual Term')
  AND s.code IN ('MATHS', 'ENG', 'HIN');

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'LKG'
  AND et.code = 'HW'
  AND t.name IN ('First Term', 'Half Yearly Term', 'Second Term', 'Annual Term')
  AND s.code IN ('MATHS', 'ENG', 'HIN');

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'LKG'
  AND et.code = 'CN'
  AND t.name IN ('First Term', 'Half Yearly Term', 'Second Term', 'Annual Term')
  AND s.code IN ('MATHS', 'ENG', 'HIN');

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'LKG'
  AND et.code = 'ORAL_PRAC'
  AND t.name IN ('First Term', 'Half Yearly Term', 'Second Term', 'Annual Term')
  AND s.code IN ('ENG_ORAL', 'ENG_RHYMES','HIN_ORAL', 'HIN_RHYMES','MATHS_ORAL');

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'LKG'
  AND et.code = 'ACTIVITY'
  AND t.name IN ( 'Half Yearly Term', 'Annual Term')
  AND s.code IN ('WELL_DRESSED', 'DRAW','PT', 'WORKSHEET');

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'LKG'
  AND et.code = 'ACTIVITY'
  AND t.name IN ('First Term', 'Second Term')
  AND s.code IN ('WELL_DRESSED', 'DRAW','PT');

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'LKG'
  AND et.code = 'CLASS_PER'
  AND t.name IN ('First Term', 'Half Yearly Term', 'Second Term', 'Annual Term')
  AND s.code IN ('CLASS_PERF');

-- UKG

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 50
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'UKG'
  AND et.code = 'WRITT'
  AND t.name IN ('First Term', 'Half Yearly Term', 'Second Term', 'Annual Term')
  AND s.code IN ('MATHS', 'ENG', 'HIN', 'GS');

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'UKG'
  AND et.code = 'HW'
  AND t.name IN ('First Term', 'Half Yearly Term', 'Second Term', 'Annual Term')
  AND s.code IN ('MATHS', 'ENG', 'HIN', 'GS');

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'UKG'
  AND et.code = 'CN'
  AND t.name IN ('First Term', 'Half Yearly Term', 'Second Term', 'Annual Term')
  AND s.code IN ('MATHS', 'ENG', 'HIN', 'GS');

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'UKG'
  AND et.code = 'ORAL_PRAC'
  AND t.name IN ('First Term', 'Half Yearly Term', 'Second Term', 'Annual Term')
  AND s.code IN ('ENG_ORAL', 'ENG_RHYMES','HIN_ORAL', 'HIN_RHYMES','MATHS_ORAL', 'GS_ORAL');

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'UKG'
  AND et.code = 'ACTIVITY'
  AND t.name IN ( 'Half Yearly Term', 'Annual Term')
  AND s.code IN ('WELL_DRESSED', 'DRAW','PT', 'WORKSHEET');

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'UKG'
  AND et.code = 'ACTIVITY'
  AND t.name IN ('First Term', 'Second Term')
  AND s.code IN ('WELL_DRESSED', 'DRAW','PT');

INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = 'UKG'
  AND et.code = 'CLASS_PER'
  AND t.name IN ('First Term', 'Half Yearly Term', 'Second Term', 'Annual Term')
  AND s.code IN ('CLASS_PERF');



-- Class 1st --
-- unit 1 written exames
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 20
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '1'
  AND et.code = 'UT'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- Unit test 1 grade exams
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '1'
  AND et.code = 'UT'
  AND t.name IN ('First Term')
  AND s.code IN ('DRAW');


-- half yearly written exams
-- Class performance
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '1'
  AND et.code = 'CLASS_PER'
  AND t.name IN ('Half Yearly Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- Copy work
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '1'
  AND et.code = 'COPY_WORK'
  AND t.name IN ('Half Yearly Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- oral pactical
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 20
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '1'
  AND et.code = 'ORAL_PRAC'
  AND t.name IN ('Half Yearly Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

--Written
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 40
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '1'
  AND et.code = 'WRITT'
  AND t.name IN ('Half Yearly Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- half yearly grade exams
-- Written 
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '1'
  AND et.code = 'WRITT'
  AND t.name IN ('Half Yearly Term')
  AND s.code IN ('DRAW', 'PT');


-- unit 2 written exames
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 20
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '1'
  AND et.code = 'UT'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- Unit test 2 grade exams
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '1'
  AND et.code = 'UT'
  AND t.name IN ('Second Term')
  AND s.code IN ('DRAW');


-- Annual written exams
-- Class performance
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '1'
  AND et.code = 'CLASS_PER'
  AND t.name IN ('Annual Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- copy work
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '1'
  AND et.code = 'COPY_WORK'
  AND t.name IN ('Annual Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- oral pactical
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 20
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '1'
  AND et.code = 'ORAL_PRAC'
  AND t.name IN ('Annual Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

--Written
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 40
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '1'
  AND et.code = 'WRITT'
  AND t.name IN ('Annual Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- Annual grade exams
-- Written 
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '1'
  AND et.code = 'WRITT'
  AND t.name IN ('Annual Term')
  AND s.code IN ('DRAW', 'PT');


-- Class 2nd --
-- unit 2 written exames
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 20
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '2'
  AND et.code = 'UT'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- Unit test 2 grade exams
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '2'
  AND et.code = 'UT'
  AND t.name IN ('First Term')
  AND s.code IN ('DRAW');


-- half yearly written exams
-- Class performance
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '2'
  AND et.code = 'CLASS_PER'
  AND t.name IN ('Half Yearly Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- Copy work
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '2'
  AND et.code = 'COPY_WORK'
  AND t.name IN ('Half Yearly Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- oral pactical
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 20
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '2'
  AND et.code = 'ORAL_PRAC'
  AND t.name IN ('Half Yearly Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

--Written
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 40
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '2'
  AND et.code = 'WRITT'
  AND t.name IN ('Half Yearly Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- half yearly grade exams
-- Written 
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '2'
  AND et.code = 'WRITT'
  AND t.name IN ('Half Yearly Term')
  AND s.code IN ('DRAW', 'PT');

-- unit 2 written exames
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 20
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '2'
  AND et.code = 'UT'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- Unit test 2 grade exams
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '2'
  AND et.code = 'UT'
  AND t.name IN ('Second Term')
  AND s.code IN ('DRAW');


-- Annual written exams
-- Copy work
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '2'
  AND et.code = 'COPY_WORK'
  AND t.name IN ('Annual Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- class performance
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '2'
  AND et.code = 'CLASS_PER'
  AND t.name IN ('Annual Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- oral pactical
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 20
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '2'
  AND et.code = 'ORAL_PRAC'
  AND t.name IN ('Annual Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

--Written
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 40
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '2'
  AND et.code = 'WRITT'
  AND t.name IN ('Annual Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- Annual grade exams
-- Written 
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '2'
  AND et.code = 'WRITT'
  AND t.name IN ('Annual Term')
  AND s.code IN ('DRAW', 'PT');


-- Class 3--
-- First term
-- Unit test
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 20
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '3'
  AND et.code = 'UT'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- Copy work
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '3'
  AND et.code = 'COPY_WORK'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- Class performance
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '3'
  AND et.code = 'CLASS_PER'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- half yearly exam (written)
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 60
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '3'
  AND et.code = 'HALF_YEARLY'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- First term grades
-- unit test - 1
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '3'
  AND et.code = 'UT'
  AND t.name IN ('First Term')
  AND s.code IN ('DRAW');

-- Half yearly
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '3'
  AND et.code = 'HALF_YEARLY'
  AND t.name IN ('First Term')
  AND s.code IN ('DRAW', 'PT');

-- second term
-- Unit test - 2
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 20
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '3'
  AND et.code = 'UT'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- Copy work
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '3'
  AND et.code = 'COPY_WORK'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- Class performance
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '3'
  AND et.code = 'CLASS_PER'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- Annual Exam (written)
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 60
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '3'
  AND et.code = 'ANNUAL'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- First term grades
-- unit test - 2
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '3'
  AND et.code = 'UT'
  AND t.name IN ('Second Term')
  AND s.code IN ('DRAW');

-- Annual
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '3'
  AND et.code = 'ANNUAL'
  AND t.name IN ('Second Term')
  AND s.code IN ('DRAW', 'PT');


-- Class 4--
-- First term
-- Unit test
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 20
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '4'
  AND et.code = 'UT'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- Copy work
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '4'
  AND et.code = 'COPY_WORK'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- Class performance
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '4'
  AND et.code = 'CLASS_PER'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- half yearly exam (written)
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 60
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '4'
  AND et.code = 'HALF_YEARLY'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- First term grades
-- unit test - 1
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '4'
  AND et.code = 'UT'
  AND t.name IN ('First Term')
  AND s.code IN ('DRAW');

-- Half yearly
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '4'
  AND et.code = 'HALF_YEARLY'
  AND t.name IN ('First Term')
  AND s.code IN ('DRAW', 'PT');

-- second term
-- Unit test - 2
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 20
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '4'
  AND et.code = 'UT'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- Copy work
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '4'
  AND et.code = 'COPY_WORK'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- Class performance
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '4'
  AND et.code = 'CLASS_PER'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- Annual Exam (written)
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 60
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '4'
  AND et.code = 'ANNUAL'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK');

-- First term grades
-- unit test - 2
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '4'
  AND et.code = 'UT'
  AND t.name IN ('Second Term')
  AND s.code IN ('DRAW');

-- Annual
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '4'
  AND et.code = 'ANNUAL'
  AND t.name IN ('Second Term')
  AND s.code IN ('DRAW', 'PT');


-- Class 5--
-- First term
-- Unit test
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 20
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '5'
  AND et.code = 'UT'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK', 'SANS');

-- Copy work
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '5'
  AND et.code = 'COPY_WORK'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK', 'SANS');

-- Class performance
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '5'
  AND et.code = 'CLASS_PER'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK', 'SANS');

-- half yearly exam (written)
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 60
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '5'
  AND et.code = 'HALF_YEARLY'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK', 'SANS');

-- First term grades
-- unit test - 1
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '5'
  AND et.code = 'UT'
  AND t.name IN ('First Term')
  AND s.code IN ('DRAW');

-- Half yearly
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '5'
  AND et.code = 'HALF_YEARLY'
  AND t.name IN ('First Term')
  AND s.code IN ('DRAW', 'PT');

-- second term
-- Unit test - 2
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 20
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '5'
  AND et.code = 'UT'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK', 'SANS');

-- Copy work
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '5'
  AND et.code = 'COPY_WORK'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK', 'SANS');

-- Class performance
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '5'
  AND et.code = 'CLASS_PER'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK', 'SANS');

-- Annual Exam (written)
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 60
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '5'
  AND et.code = 'ANNUAL'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'MATHS', 'SST', 'EVS', 'COMP', 'GK', 'SANS');

-- First term grades
-- unit test - 2
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '5'
  AND et.code = 'UT'
  AND t.name IN ('Second Term')
  AND s.code IN ('DRAW');

-- Annual
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '5'
  AND et.code = 'ANNUAL'
  AND t.name IN ('Second Term')
  AND s.code IN ('DRAW', 'PT');


-- Class 6--
-- First term
-- Unit test
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 20
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '6'
  AND et.code = 'UT'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- Copy work
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '6'
  AND et.code = 'COPY_WORK'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- Class performance
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '6'
  AND et.code = 'CLASS_PER'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- half yearly exam (written)
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 60
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '6'
  AND et.code = 'HALF_YEARLY'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- First term grades
-- unit test - 1
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '6'
  AND et.code = 'UT'
  AND t.name IN ('First Term')
  AND s.code IN ('DRAW', 'PT');

-- Half yearly
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '6'
  AND et.code = 'HALF_YEARLY'
  AND t.name IN ('First Term')
  AND s.code IN ('DRAW', 'PT');

-- second term
-- Unit test - 2
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 20
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '6'
  AND et.code = 'UT'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- Copy work
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '6'
  AND et.code = 'COPY_WORK'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- Class performance
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '6'
  AND et.code = 'CLASS_PER'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- Annual Exam (written)
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 60
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '6'
  AND et.code = 'ANNUAL'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- First term grades
-- unit test - 2
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '6'
  AND et.code = 'UT'
  AND t.name IN ('Second Term')
  AND s.code IN ('DRAW', 'PT');

-- Annual
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '6'
  AND et.code = 'ANNUAL'
  AND t.name IN ('Second Term')
  AND s.code IN ('DRAW', 'PT');


-- Class 7--
-- First term
-- Unit test
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 20
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '7'
  AND et.code = 'UT'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- Copy work
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '7'
  AND et.code = 'COPY_WORK'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- Class performance
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '7'
  AND et.code = 'CLASS_PER'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- half yearly exam (written)
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 60
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '7'
  AND et.code = 'HALF_YEARLY'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- First term grades
-- unit test - 1
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '7'
  AND et.code = 'UT'
  AND t.name IN ('First Term')
  AND s.code IN ('DRAW', 'PT');

-- Half yearly
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '7'
  AND et.code = 'HALF_YEARLY'
  AND t.name IN ('First Term')
  AND s.code IN ('DRAW', 'PT');

-- second term
-- Unit test - 2
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 20
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '7'
  AND et.code = 'UT'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- Copy work
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '7'
  AND et.code = 'COPY_WORK'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- Class performance
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '7'
  AND et.code = 'CLASS_PER'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- Annual Exam (written)
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 60
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '7'
  AND et.code = 'ANNUAL'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- First term grades
-- unit test - 2
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '7'
  AND et.code = 'UT'
  AND t.name IN ('Second Term')
  AND s.code IN ('DRAW', 'PT');

-- Annual
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '7'
  AND et.code = 'ANNUAL'
  AND t.name IN ('Second Term')
  AND s.code IN ('DRAW', 'PT');


-- Class 8--
-- First term
-- Unit test
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 20
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '8'
  AND et.code = 'UT'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- Copy work
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '8'
  AND et.code = 'COPY_WORK'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- Class performance
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '8'
  AND et.code = 'CLASS_PER'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- half yearly exam (written)
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 60
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '8'
  AND et.code = 'HALF_YEARLY'
  AND t.name IN ('First Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- First term grades
-- unit test - 1
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '8'
  AND et.code = 'UT'
  AND t.name IN ('First Term')
  AND s.code IN ('DRAW', 'PT');

-- Half yearly
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '8'
  AND et.code = 'HALF_YEARLY'
  AND t.name IN ('First Term')
  AND s.code IN ('DRAW', 'PT');

-- second term
-- Unit test - 2
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 20
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '8'
  AND et.code = 'UT'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- Copy work
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '8'
  AND et.code = 'COPY_WORK'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- Class performance
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 10
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '8'
  AND et.code = 'CLASS_PER'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- Annual Exam (written)
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type, max_marks)
SELECT g.id, e.id, s.id, 'MARKS', 60
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '8'
  AND et.code = 'ANNUAL'
  AND t.name IN ('Second Term')
  AND s.code IN ('ENG_LIT', 'ENG_LANG', 'HIN', 'SANS', 'MATHS', 'SCI_BIO', 'SCI_PHY_CHEM', 'GEO', 'HIST_CIV', 'COMP', 'GK');

-- First term grades
-- unit test - 2
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '8'
  AND et.code = 'UT'
  AND t.name IN ('Second Term')
  AND s.code IN ('DRAW', 'PT');

-- Annual
INSERT INTO assessment_scheme (grade_id, exam_id, subject_id, evaluation_type)
SELECT g.id, e.id, s.id, 'GRADE'
FROM grades g
JOIN exams e ON true
JOIN exam_types et ON e.exam_type_id = et.id
JOIN terms t ON e.term_id = t.id
JOIN subjects s ON true
WHERE g.name = '8'
  AND et.code = 'ANNUAL'
  AND t.name IN ('Second Term')
  AND s.code IN ('DRAW', 'PT');
