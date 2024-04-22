CREATE TABLE tag_category (
  id SERIAL NOT NULL PRIMARY KEY,
  name VARCHAR(255) NOT NULL UNIQUE,
  created_by UUID NOT NULL,
  timestamp TIMESTAMP(0) WITH TIME ZONE NOT NULL,
  FOREIGN KEY (created_by) REFERENCES account(id)
);

ALTER TABLE
    tag ADD CONSTRAINT tag_category_id_foreign FOREIGN KEY(category_id) REFERENCES tag_category(id);
