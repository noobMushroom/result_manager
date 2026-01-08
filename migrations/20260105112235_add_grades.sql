-- Add migration script here
CREATE TABLE grades (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,        
    sort_order INT NOT NULL,   
    UNIQUE (name),
    UNIQUE (sort_order)
);

INSERT INTO grades (name, sort_order) VALUES
  ('NURSERY', 0),
  ('LKG', 1),
  ('UKG', 2),
  ('1', 3),
  ('2', 4),
  ('3', 5),
  ('4', 6),
  ('5', 7),
  ('6', 8),
  ('7', 9),
  ('8', 10);
