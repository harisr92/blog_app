CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR(225) NOT NULL,
  body TEXT NOT NULL,
  status VARCHAR(50) NOT NULL DEFAULT "draft"
)
