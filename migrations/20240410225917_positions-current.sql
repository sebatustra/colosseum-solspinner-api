-- Add migration script here
BEGIN;

ALTER TABLE positions
RENAME COLUMN quantity TO initial_quantity;

ALTER TABLE positions
ADD COLUMN current_quantity DOUBLE PRECISION;

UPDATE positions
SET current_quantity = initial_quantity;

ALTER TABLE positions
ALTER COLUMN current_quantity SET NOT NULL;

COMMIT;