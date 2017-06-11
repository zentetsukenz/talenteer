CREATE TABLE talents (
  id              SERIAL PRIMARY KEY,
  uid             uuid DEFAULT uuid_generate_v4(),
  username        VARCHAR NOT NULL,
  password_digest VARCHAR NOT NULL,
  data            JSON
)
