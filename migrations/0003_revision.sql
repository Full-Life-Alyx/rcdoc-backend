CREATE TABLE revision (
  id UUID NOT NULL PRIMARY KEY,
  version INTEGER NOT NULL,
  document_id UUID NOT NULL,
  author UUID NOT NULL,
  timestamp TIMESTAMP(0) WITH TIME ZONE NOT NULL,
  FOREIGN KEY (author) REFERENCES account(id),
  FOREIGN KEY (document_id) REFERENCES document(id)
);

