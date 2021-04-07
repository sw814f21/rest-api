DROP TABLE smileyreports;

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
    name VARCHAR NOT NULL,
    latest_control INTEGER,
    second_latest_control INTEGER,
    third_latest_control INTEGER,
    fourth_latest_control INTEGER
);

INSERT INTO temprestaurants(id, city, cvr, latitude, longitude, pnr, address, url, zipcode, name) SELECT * FROM restaurants;
DROP TABLE restaurants;
ALTER TABLE temprestaurants RENAME TO restaurants;
