CREATE TABLE talents (
  id              SERIAL PRIMARY KEY,
  username        VARCHAR NOT NULL,
  password_digest VARCHAR NOT NULL,
  data            JSONB,
  created_at      TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at      TIMESTAMP WITH TIME ZONE NOT NULL
)
