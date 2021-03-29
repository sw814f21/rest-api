CREATE TABLE IF NOT EXISTS favoritestemp (
  restaurant_id INTEGER NOT NULL,
  token_id TEXT NOT NULL,
  FOREIGN KEY (token_id) REFERENCES users(token_id) ON DELETE CASCADE,
  FOREIGN KEY (restaurant_id) REFERENCES restaurants(id) ON DELETE CASCADE,
  PRIMARY KEY (restaurant_id, token_id)
);
INSERT INTO favoritestemp(restaurant_id, token_id) SELECT * FROM favorites;
DROP TABLE favorites;
ALTER TABLE favoritestemp RENAME TO favorites;


