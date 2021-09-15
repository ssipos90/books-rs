-- Add up migration script here

ALTER TABLE authors
  ADD COLUMN genre genre NOT NULL;