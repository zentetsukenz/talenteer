CREATE TABLE talents (
  id              SERIAL PRIMARY KEY,
  name            VARCHAR NOT NULL,
  username        VARCHAR NOT NULL,
  password_digest VARCHAR NOT NULL
)
