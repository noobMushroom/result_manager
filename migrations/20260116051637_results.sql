-- Add migration script here
CREATE TABLE results (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    student_id UUID NOT NULL REFERENCES students(id),
    assessment_id UUID NOT NULL REFERENCES assessment_scheme(id),

    marks_obtained INTEGER,
    grade TEXT,

    result_status TEXT NOT NULL DEFAULT 'PRESENT',
    -- PRESENT | ABSENT | MEDICAL

    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),

    UNIQUE (student_id, assessment_id),

    CHECK (
        (
            result_status = 'PRESENT'
            AND (
                (marks_obtained IS NOT NULL AND grade IS NULL)
             OR (marks_obtained IS NULL AND grade IS NOT NULL)
            )
        )
        OR
        (
            result_status IN ('ABSENT', 'MEDICAL')
            AND marks_obtained IS NULL
            AND grade IS NULL
        )
    )
);
