CREATE TABLE account (
    id UUID NOT NULL PRIMARY KEY,
    nickname VARCHAR(64) NOT NULL,
    identifier VARCHAR(16) NOT NULL,
    is_admin BOOLEAN NOT NULL
);
