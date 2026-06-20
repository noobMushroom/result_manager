-- Add migration script here
CREATE TABLE subjects (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    evaluation_type TEXT NOT NULL,
    UNIQUE (name),
    UNIQUE (code)
);


INSERT INTO subjects (name, code, evaluation_type) VALUES
  ('Mathematics', 'MATHS', 'MARKS'),
  ('English', 'ENG', 'MARKS'),
  ('Hindi', 'HIN', 'MARKS'),

  ('English Oral', 'ENG_ORAL', 'GRADE'),
  ('English Rhymes', 'ENG_RHYMES', 'GRADE'),
  ('Hindi Oral', 'HIN_ORAL', 'GRADE'),
  ('Hindi Rhymes', 'HIN_RHYMES', 'GRADE'),
  ('Mathematics Oral', 'MATHS_ORAL', 'GRADE'),

  ('Class Performance', 'CLASS_PERF', 'GRADE'),
  ('Well Dressed', 'WELL_DRESSED', 'GRADE'),
  ('Drawing', 'DRAW', 'GRADE'),
  ('Physical Training', 'PT', 'GRADE'),

  ('Worksheet Activity', 'WORKSHEET', 'GRADE'),
  ('Activity Work', 'ACTIVITY', 'GRADE'),

  ('General Science', 'GS', 'MARKS'),
  ('General Science Oral', 'GS_ORAL', 'GRADE'),

  ('English Literature', 'ENG_LIT', 'MARKS'),
  ('English Language', 'ENG_LANG', 'MARKS'),

  ('Social Science', 'SS', 'MARKS'),
  ('Environmental Science', 'EVS', 'MARKS'),
  ('Computer', 'COMP', 'MARKS'),
  ('General Knowledge', 'GK', 'MARKS'),
  ('Sanskrit', 'SANS', 'MARKS'),
  ('Science', 'SCI', 'MARKS');

