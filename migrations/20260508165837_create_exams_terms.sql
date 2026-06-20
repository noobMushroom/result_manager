-- Add migration script here
CREATE TABLE terms (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    sort_order INT NOT NULL,
    UNIQUE (name),
    UNIQUE (sort_order)
);

INSERT INTO terms (name, sort_order) VALUES
    ('First Term', 1),
    ('Second Term', 2),
    ('Annual Term', 3),
    ('Half Yearly Term', 4);

CREATE TABLE exam_types (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    UNIQUE (name),
    UNIQUE (code)
);

INSERT INTO exam_types (name, code) VALUES
  ('Unit Test 1', 'UT-I'),
  ('Unit Test 2', 'UT-II'),
  ('Copy Work', 'COPY_WORK'),
  ('Activity', 'ACTIVITY'),
  ('Half Yearly', 'HALF_YEARLY'),
  ('Oral/Practical', 'ORAL_PRAC'),
  ('Written', 'WRITT'),
  ('Copy Neatness', 'CN'),
  ('Homework', 'HW'),
  ('Annual', 'ANNUAL');

