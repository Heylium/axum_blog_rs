CREATE TABLE categories
(
    id     SERIAL PRIMARY KEY,
    name   VARCHAR(100) NOT NULL,
    is_del BOOLEAN      NOT NULL DEFAULT FALSE
);