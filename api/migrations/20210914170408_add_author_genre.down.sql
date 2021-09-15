-- Add down migration script here

ALTER TABLE authors
  DROP COLUMN genre;