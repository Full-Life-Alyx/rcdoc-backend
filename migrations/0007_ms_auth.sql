CREATE TABLE ms_auth (
    id SERIAL PRIMARY KEY,
    account_id UUID NOT NULL,
    -- Honestly, I do not know how big it is, I am scared to put it as 44 (which was mine)
    subject VARCHAR(64) UNIQUE, 
  FOREIGN KEY (account_id) REFERENCES account(id)
);

