use serde::{Serialize, Deserialize};
use crate::schema::{supermarkets, checkins};

use diesel_geometry::data_types::PgPoint;
use chrono::NaiveDateTime;

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name="supermarkets"]
pub struct Supermarket {
    pub id: String,
    pub name: Option<String>,
    pub lat: f64,
    pub lon: f64,
    pub housenumber: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub brand: Option<String>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name="checkins"]
pub struct NewCheckin {
    pub gps_lat: PgPoint,
    pub location_name: String,
    pub crowded_level: i32,
    pub user_id: String,
    pub client_id: String,
    pub created_at: NaiveDateTime,
}
