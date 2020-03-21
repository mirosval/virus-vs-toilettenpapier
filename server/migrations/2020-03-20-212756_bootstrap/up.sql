-- A collection of all the locations that we are aware of.
-- A geo search can then be performed on this.
CREATE TABLE IF NOT EXISTS supermarkets (
    id TEXT PRIMARY KEY,
    name TEXT DEFAULT NULL,
    lat DOUBLE PRECISION NOT NULL,
    lon DOUBLE PRECISION NOT NULL,
    housenumber TEXT DEFAULT NULL,
    city TEXT DEFAULT NULL,
    country TEXT DEFAULT NULL,
    brand TEXT DEFAULT NULL
);

