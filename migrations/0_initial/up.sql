-- Your SQL goes here

CREATE TABLE IF NOT EXISTS restaurant (
  id INTEGER NOT NULL PRIMARY KEY,
  smiley_restaurant_id INTEGER NOT NULL,
  name VARCHAR NOT NULL,
  address VARCHAR NOT NULL,
  zipcode VARCHAR NOT NULL,
  city VARCHAR NOT NULL,
  cvr VARCHAR NOT NULL,
  pnr VARCHAR NOT NULL,
  latitude REAL NOT NULL,
  longitude REAL NOT NULL
);

CREATE TABLE IF NOT EXISTS smiley_report (
  id INTEGER NOT NULL PRIMARY KEY,
  restaurant_id INTEGER NOT NULL,
  smiley INTEGER NOT NULL,
  report_id VARCHAR NOT NULL,
  date VARCHAR NOT NULL,
  FOREIGN KEY (restaurant_id) REFERENCES restaurant(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS subscription (
  id INTEGER NOT NULL PRIMARY KEY,
  restaurant_id INTEGER NOT NULL,
  token VARCHAR NOT NULL,
  FOREIGN KEY (restaurant_id) REFERENCES restaurant(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS notification_history (
  id INTEGER NOT NULL PRIMARY KEY,
  subscription_id INTEGER NOT NULL,
  timestamp VARCHAR NOT NULL,
  data VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  body VARCHAR NOT NULL,
  FOREIGN KEY (subscription_id) REFERENCES subscription(id) ON DELETE CASCADE
);