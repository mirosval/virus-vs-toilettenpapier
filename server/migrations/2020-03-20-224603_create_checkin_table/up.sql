-- Your SQL goes here

CREATE TABLE IF NOT EXISTS missing_goods (
    id SERIAL PRIMARY KEY,
    checkin_id
)

CREATE TABLE IF NOT EXISTS checkin (
    id SERIAL PRIMARY KEY,
    gps_lat 
    location_name TEXT NOT NULL,
    crowded_level NUMERIC NOT NULL, 
    user_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
)
