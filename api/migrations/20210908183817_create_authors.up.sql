-- Add up migration script here

CREATE TABLE authors (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
)
