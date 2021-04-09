CREATE TABLE IF NOT EXISTS smileyreports (
  id INTEGER NOT NULL PRIMARY KEY,
  restaurant_id INTEGER NOT NULL,
  rating INTEGER NOT NULL,
  date TEXT NOT NULL,
  report_id TEXT NOT NULL,
  FOREIGN KEY (restaurant_id) REFERENCES restaurants(id) ON DELETE CASCADE

);


CREATE TABLE IF NOT EXISTS temprestaurants (
    id INTEGER NOT NULL PRIMARY KEY,
    city VARCHAR NOT NULL,
    cvr VARCHAR NOT NULL,
    latitude REAL NOT NULL,
    longitude REAL NOT NULL,
    pnr VARCHAR NOT NULL,
    address VARCHAR NOT NULL,
    url VARCHAR NOT NULL,
    zipcode VARCHAR NOT NULL,
    name VARCHAR NOT NULL
);

INSERT INTO temprestaurants(id, city, cvr, latitude, longitude, pnr, address, url, zipcode, name) SELECT id, city, cvr, latitude, longitude, pnr, address, url, zipcode, name FROM restaurants;
DROP TABLE restaurants;
ALTER TABLE temprestaurants RENAME TO restaurants;
