CREATE TABLE restaurants (
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
)