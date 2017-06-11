CREATE UNIQUE INDEX CONCURRENTLY talents_username ON talents USING btree (
  username
)
