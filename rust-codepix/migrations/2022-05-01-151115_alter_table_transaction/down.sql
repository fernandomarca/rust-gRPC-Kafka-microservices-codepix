-- This file should undo anything in `up.sql`
ALTER TABLE transaction
  DROP COLUMN pix_key_kind;