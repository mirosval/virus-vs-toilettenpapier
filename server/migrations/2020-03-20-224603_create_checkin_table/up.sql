-- Your SQL goes here

CREATE TABLE IF NOT EXISTS checkins (
    id SERIAL PRIMARY KEY,
    gps POINT NOT NULL, 
    location_name TEXT NOT NULL,
    crowded_level INTEGER NOT NULL, 
    missing_goods TEXT[] NOT NULL DEFAULT '{}',
    user_id TEXT NOT NULL,
    client_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL
);

