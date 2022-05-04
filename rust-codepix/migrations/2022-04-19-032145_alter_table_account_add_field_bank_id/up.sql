-- Your SQL goes here-- pub struct AccountModel {
--   pub id: String,
--   pub owner_name: String,
--   pub number: String,
--   pub bank_id: String,
--   pub pix_keys: Vec<PixKeyModel>,
--   pub created_at: DateTime<Utc>,
--   pub updated_at: DateTime<Utc>,
-- }
ALTER TABLE account
  ADD COLUMN bank_id VARCHAR REFERENCES bank(id) ON DELETE CASCADE;

ALTER TABLE account
ALTER COLUMN bank_id SET NOT NULL;