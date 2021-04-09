CREATE TABLE IF NOT EXISTS users (
  token_id TEXT NOT NULL,
  notifications INTEGER NOT NULL,
  PRIMARY KEY (token_id)
);

CREATE TABLE IF NOT EXISTS favorites (
  resturant_id INTEGER NOT NULL,
  token_id TEXT,
  FOREIGN KEY (token_id) REFERENCES users(token_id) ON DELETE CASCADE,
  PRIMARY KEY (resturant_id, token_id)
);