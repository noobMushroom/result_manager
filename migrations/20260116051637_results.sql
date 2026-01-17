-- Add migration script here
CREATE TABLE results (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    student_id UUID NOT NULL REFERENCES students(id),
    assessment_id UUID NOT NULL REFERENCES assessment_scheme(id),

    marks_obtained INTEGER,
    grade TEXT,

    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),

    UNIQUE (student_id, assessment_id),

    CHECK (
        (marks_obtained IS NOT NULL AND grade IS NULL)
     OR (marks_obtained IS NULL AND grade IS NOT NULL)
    )
);
