ALTER TABLE users
  ADD COLUMN encrypted_password VARCHAR(256) NOT NULL,
  ADD COLUMN password_salt VARCHAR(50) NOT NULL;
