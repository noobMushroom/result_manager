-- Add migration script here
CREATE TABLE subjects (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    UNIQUE (name),
    UNIQUE (code)
);


INSERT INTO subjects (name, code) VALUES
  ('Mathematics', 'MATHS'),
  ('English', 'ENG'),
  ('Hindi', 'HIN'),

  ('English Oral', 'ENG_ORAL'),
  ('English Rhymes', 'ENG_RHYMES'),
  ('Hindi Oral', 'HIN_ORAL'),
  ('Hindi Rhymes', 'HIN_RHYMES'),
  ('Mathematics Oral', 'MATHS_ORAL'),

  ('Class Performance', 'CLASS_PERF'),
  ('Well Dressed', 'WELL_DRESSED'),
  ('Drawing', 'DRAW'),
  ('Physical Training', 'PT'),

  ('Worksheet Activity', 'WORKSHEET'),
  ('Activity Work', 'ACTIVITY'),

  ('General Science', 'GS'),
  ('General Science Oral', 'GS_ORAL'),

  ('English Literature', 'ENG_LIT'),
  ('English Language', 'ENG_LANG'),

  ('Social Studies', 'SST'),
  ('Environmental Science', 'EVS'),
  ('Computer', 'COMP'),
  ('General Knowledge', 'GK'),
  ('Sanskrit', 'SANS'),

  ('Science Biology', 'SCI_BIO'),
  ('Science Physics Chemistry', 'SCI_PHY_CHEM'),

  ('Geography', 'GEO'),
  ('History Civics', 'HIST_CIV');
