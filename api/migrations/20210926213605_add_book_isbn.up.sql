-- Add up migration script here

ALTER TABLE books ADD COLUMN isbn char(13) NOT NULL;
