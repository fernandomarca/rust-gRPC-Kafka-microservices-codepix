-- pub struct BankModel {
--   pub id: String,
--   pub code: String,
--   pub name: String,
--   pub accounts: Vec<AccountModel>,
--   pub created_at: NaiveDateTime,
--   pub updated_at: NaiveDateTime,
-- }
CREATE TABLE bank(
  id VARCHAR NOT NULL PRIMARY KEY,
  code VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  created_at timestamp NOT NULL DEFAULT NOW(),
  updated_at timestamp NOT NULL DEFAULT NOW(),
  accounts TEXT[]
);

CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON bank
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();