-- Add migration script here
CREATE TABLE results (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),

  student_id uuid NOT NULL,
  grade_id uuid NOT NULL,

  subject_id uuid NOT NULL,
  exam_type_id uuid NOT NULL,
  term_id uuid NOT NULL,

  marks_obtained integer,
  grade_obtained TEXT,

  status TEXT NOT NULL DEFAULT 'present',

  UNIQUE (student_id, subject_id, exam_type_id, term_id),

  FOREIGN KEY (student_id) REFERENCES students(id),
  FOREIGN KEY (grade_id) REFERENCES grades(id),
  FOREIGN KEY (subject_id) REFERENCES subjects(id),
  FOREIGN KEY (exam_type_id) REFERENCES exam_types(id),
  FOREIGN KEY (term_id) REFERENCES terms(id),

  FOREIGN KEY (grade_id, exam_type_id, term_id)
    REFERENCES grade_terms(grade_id, exam_type_id, term_id),

  CHECK (marks_obtained IS NULL OR marks_obtained >= 0),

  CHECK (
    (status = 'present' AND (
        (marks_obtained IS NOT NULL AND grade_obtained IS NULL) OR
        (marks_obtained IS NULL AND grade_obtained IS NOT NULL)
    ))
    OR
    (status IN ('absent', 'medical', 'not_evaluated')
      AND marks_obtained IS NULL
      AND grade_obtained IS NULL)
  )
);
