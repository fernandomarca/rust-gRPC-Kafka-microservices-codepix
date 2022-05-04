-- Your SQL goes here
ALTER TABLE transaction
  ADD COLUMN pix_key_kind VARCHAR;

ALTER TABLE transaction
ALTER COLUMN pix_key_kind SET NOT NULL;