CREATE TABLE users (
  token_id TEXT,
  notifications int,
  PRIMARY KEY (token_id)
);

CREATE TABLE favorites (
  resturant_id int,
  user_id TEXT,
  FOREIGN KEY (user_id) REFERENCES users(token_id) ON DELETE CASCADE,
  PRIMARY KEY (resturant_id, user_id)
);