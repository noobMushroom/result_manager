-- Add migration script here
CREATE TABLE students (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),

    grade_id uuid NOT NULL,

    name TEXT NOT NULL,
    father_name TEXT NOT NULL,

    admission_no INTEGER NOT NULL UNIQUE,

    date_of_birth DATE NOT NULL,

    section_id uuid NULL,

    created_at timestamptz NOT NULL,

    FOREIGN KEY (grade_id) REFERENCES grades(id),
    FOREIGN KEY (section_id) REFERENCES sections(id)
);
