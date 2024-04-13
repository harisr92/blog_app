ALTER TABLE users
  DROP COLUMN encrypted_password,
  DROP COLUMN password_salt;
