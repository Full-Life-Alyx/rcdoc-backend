CREATE TABLE account (
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    nickname VARCHAR(64) NOT NULL,
    identifier VARCHAR(16) NOT NULL,
    is_admin BOOLEAN NOT NULL
);
