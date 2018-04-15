CREATE TABLE Issues(
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  description VARCHAR,
  created DATE NOT NULL
)