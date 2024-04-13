-- Your SQL goes here
ALTER TABLE posts
  ADD COLUMN user_id BIGINT UNSIGNED NOT NULL REFERENCES users(id);
