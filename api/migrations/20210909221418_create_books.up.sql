-- Add up migration script here

CREATE TABLE books (
  id SERIAL PRIMARY KEY,
  author_id INT NOT NULL,
  title VARCHAR NOT NULL,
  genre genre NOT NULL,
  CONSTRAINT fk_book_author
    FOREIGN KEY(author_id) 
    REFERENCES authors(id)
)
