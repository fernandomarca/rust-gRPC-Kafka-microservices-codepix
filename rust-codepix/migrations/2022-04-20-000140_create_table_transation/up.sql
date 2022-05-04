-- pub struct TransactionModel {
--   pub id: Option<String>,
--   pub amount: i64,
--   pub status: String,
--   pub description: String,
--   pub created_at: NaiveDateTime,
--   pub updated_at: NaiveDateTime,
--   // relations
--   pub account_from: AccountModel,
--   pub account_from_id: String,
--   // relations
--   pub pix_key_to: PixKeyModel,
--   pub pix_key_id_to: String,
-- }

CREATE TABLE transaction (
  id VARCHAR NOT NULL PRIMARY KEY,
  amount NUMERIC NOT NULL,
  status VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  created_at timestamp NOT NULL DEFAULT NOW(),
  updated_at timestamp NOT NULL DEFAULT NOW(),
  account_from_id VARCHAR NOT NULL REFERENCES account(id),
  pix_key_id_to VARCHAR NOT NULL REFERENCES pixkey(id)
);

CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON transaction
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();