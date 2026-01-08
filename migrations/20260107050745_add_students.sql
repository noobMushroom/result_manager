-- Add migration script here
CREATE TABLE students (
    id uuid PRIMARY KEY,

    grade_id uuid NOT NULL,

    name TEXT NOT NULL,
    father_name TEXT NOT NULL,

    admission_no INTEGER NOT NULL UNIQUE,

    date_of_birth DATE NOT NULL,

    FOREIGN KEY (grade_id) REFERENCES grades(id)
);
