use serde::{Serialize, Deserialize};
use crate::schema::{location, checkins};

#[derive(Eq, PartialEq, Debug, Queryable)]
pub struct Location {
    pub id: i32,
    name: String,
}

#[derive(Insertable)]
#[table_name="location"]
pub struct NewLocation<'a> {
    pub name: &'a str,
}

use diesel_geometry::data_types::PgPoint;
use chrono::NaiveDateTime;

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name="checkins"]
pub struct NewCheckin {
    pub gps: PgPoint,
    pub location_name: String,
    pub crowded_level: i32,
    pub user_id: String,
    pub client_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Checkin {
    pub id: i32,
    pub gps: PgPoint,
    pub location_name: String,
    pub crowded_level: i32,
    pub user_id: String,
    pub client_id: String,
    pub created_at: NaiveDateTime,
}
