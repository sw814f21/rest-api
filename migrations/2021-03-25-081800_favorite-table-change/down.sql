CREATE TABLE IF NOT EXISTS favoritestemp (
  resturant_id INTEGER NOT NULL,
  token_id TEXT,
  FOREIGN KEY (token_id) REFERENCES users(token_id) ON DELETE CASCADE,
  PRIMARY KEY (resturant_id, token_id)
);

INSERT INTO favoritestemp(resturant_id, token_id) SELECT * FROM favorites;
DROP TABLE favorites;
ALTER TABLE favoritestemp RENAME TO favorites;
