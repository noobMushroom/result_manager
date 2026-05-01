-- Add migration script here
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
