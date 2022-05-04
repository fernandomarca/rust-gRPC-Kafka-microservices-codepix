-- Your SQL goes here-- pub struct AccountModel {
--   pub id: String,
--   pub owner_name: String,
--   pub number: String,
--   pub bank_id: String,
--   pub pix_keys: Vec<PixKeyModel>,
--   pub created_at: DateTime<Utc>,
--   pub updated_at: DateTime<Utc>,
-- }

CREATE TABLE account(
  id VARCHAR NOT NULL PRIMARY KEY,
  owner_name VARCHAR NOT NULL,
  number VARCHAR NOT NULL,
  created_at timestamp NOT NULL DEFAULT NOW(),
  updated_at timestamp NOT NULL DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON account
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();