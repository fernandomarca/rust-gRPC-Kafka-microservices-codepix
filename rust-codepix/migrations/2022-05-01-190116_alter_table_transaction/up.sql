-- Your SQL goes here
ALTER TABLE transaction
  ADD COLUMN pix_key_key VARCHAR;

ALTER TABLE transaction
ALTER COLUMN pix_key_key SET NOT NULL;