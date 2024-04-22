CREATE TYPE operation_type AS ENUM ('add', 'remove');

CREATE TABLE tag_assignment (
  id BIGSERIAL NOT NULL PRIMARY KEY,
  tag_id INTEGER NOT NULL,
  document_id UUID NOT NULL,
  operation operation_type,
  timestamp TIMESTAMP(0) WITH TIME ZONE NOT NULL,
  assigner_id UUID NOT NULL,
  FOREIGN KEY (tag_id) REFERENCES tag(id),
  FOREIGN KEY (assigner_id) REFERENCES account(id),
  FOREIGN KEY (document_id) REFERENCES document(id)
);
