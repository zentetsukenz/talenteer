CREATE TABLE talents (
  id              SERIAL PRIMARY KEY,
  uid             uuid DEFAULT uuid_generate_v4() NOT NULL,
  username        VARCHAR NOT NULL,
  password_digest VARCHAR NOT NULL,
  data            JSON,
  created_at      TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at      TIMESTAMP WITH TIME ZONE NOT NULL
)
