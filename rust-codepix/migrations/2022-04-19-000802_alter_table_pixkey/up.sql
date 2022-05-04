CREATE TABLE pixkey(
  id VARCHAR NOT NULL PRIMARY KEY,
  kind VARCHAR NOT NULL,
  key VARCHAR NOT NULL,
  status VARCHAR NOT NULL,
  created_at timestamp NOT NULL DEFAULT NOW(),
  updated_at timestamp NOT NULL DEFAULT NOW(),
  account_id VARCHAR NOT NULL REFERENCES account(id) ON DELETE CASCADE
);

CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON pixkey
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();