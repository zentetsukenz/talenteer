CREATE TABLE talents (
  id              SERIAL PRIMARY KEY,
  username        VARCHAR NOT NULL,
  password_digest VARCHAR NOT NULL,
  data            JSON
)
