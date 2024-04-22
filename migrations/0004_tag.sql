CREATE TABLE tag (
  id SERIAL NOT NULL PRIMARY KEY,
  name VARCHAR(255) NOT NULL UNIQUE,
  -- Bind this later
  category_id INTEGER NOT NULL,
  created_by UUID NOT NULL REFERENCES account(id),
  timestamp TIMESTAMP(0) WITH TIME ZONE NOT NULL,
  year SMALLINT NULL
);
