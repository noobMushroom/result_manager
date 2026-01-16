-- Add migration script here
CREATE TABLE results (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    student_id UUID NOT NULL
        REFERENCES students(id) ON DELETE CASCADE,

    -- ties the result to the scheme
    grade_id UUID NOT NULL,
    exam_id UUID NOT NULL,
    subject_id UUID NOT NULL,

    -- actual outcome
    marks_obtained INTEGER,
    grade TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    -- one result per student per assessment
    CONSTRAINT results_unique
        UNIQUE (student_id, grade_id, exam_id, subject_id),

    -- either marks OR grade, never both
    CONSTRAINT results_value_check
        CHECK (
            (marks_obtained IS NOT NULL AND grade IS NULL)
         OR (marks_obtained IS NULL AND grade IS NOT NULL)
        ),

    -- must match an allowed assessment
    CONSTRAINT results_assessment_fk
        FOREIGN KEY (grade_id, exam_id, subject_id)
        REFERENCES assessment_scheme (grade_id, exam_id, subject_id)
);
