-- Stores a unique location that may be a supermarket
-- or a general store of some kind.
-- We expect that if one is already identified, it will be reused,
-- otherwise a new entry will be registered in this table
CREATE TABLE IF NOT EXISTS location (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

-- Stores a unique product that can be sold. For now we do not
-- diffrentiate between a single and bundled products.
-- Just like location, we reuse when one is found, otherwise create a new one.
CREATE TABLE IF NOT EXISTS products (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    reference TEXT NOT NULL
);

-- Every time a location is reported, we register the coordinates.
-- If the report maps to the same location, we enter it here, or we create
-- a new entry in the location DB and map it here
CREATE TABLE IF NOT EXISTS location_reports (
    id TEXT PRIMARY KEY,
    coordinates POINT NOT NULL,
    location_id SERIAL,
    FOREIGN KEY (location_id) REFERENCES location (id) ON DELETE CASCADE
);