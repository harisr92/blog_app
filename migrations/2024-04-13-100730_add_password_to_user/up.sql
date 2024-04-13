ALTER TABLE users
  ADD COLUMN encrypted_password VARCHAR(256),
  ADD COLUMN password_salt VARCHAR(50);
