CREATE TABLE users
(
    id              SERIAL PRIMARY KEY,
    uuid            UUID      NOT NULL,
    username        VARCHAR   NOT NULL UNIQUE,
    hashed_password VARCHAR   NOT NULL,
    email           VARCHAR   NOT NULL,
    created_at      TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMP NOT NULL DEFAULT NOW()
);