CREATE TABLE users (
  email VARCHAR(100) NOT NULL PRIMARY KEY,
  password vARCHAR(64) NOT NULL, --bcrypt hash
  created_at TIMESTAMP NOT NULL
);