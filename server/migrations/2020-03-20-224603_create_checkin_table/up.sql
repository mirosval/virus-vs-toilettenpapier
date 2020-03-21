-- Your SQL goes here

CREATE TABLE IF NOT EXISTS checkins (
    id SERIAL PRIMARY KEY,
    gps_lat POINT NOT NULL, 
    location_name TEXT NOT NULL,
    crowded_level INTEGER NOT NULL, 
    user_id TEXT NOT NULL,
    client_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS goods (
    id TEXT PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS missing_goods (
    id SERIAL PRIMARY KEY,
    checkin_id INTEGER REFERENCES checkins(id),
    good_id TEXT REFERENCES goods(id)
);

