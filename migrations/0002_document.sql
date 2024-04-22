CREATE TABLE document (
  id UUID NOT NULL PRIMARY KEY,
  timestamp TIMESTAMP(0) WITH TIME ZONE NOT NULL,
  identifier INTEGER NOT NULL,
  checked_out_by UUID NOT NULL,
  FOREIGN KEY (checked_out_by) REFERENCES account(id)
);

