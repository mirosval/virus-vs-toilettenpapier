use crate::schema::{checkins, location};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Queryable)]
pub struct Location {
    pub id: i32,
    name: String,
}

#[derive(Insertable)]
#[table_name = "location"]
pub struct NewLocation<'a> {
    pub name: &'a str,
}

use chrono::NaiveDateTime;
use diesel_geometry::data_types::PgPoint;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewJsonCheckin {
    pub gps: [f64; 2],
    pub location_name: String,
    pub crowded_level: i32,
    pub user_id: String,
    pub client_id: String,
    pub missing_goods: Vec<String>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "checkins"]
pub struct NewCheckin {
    pub gps: PgPoint,
    pub location_name: String,
    pub crowded_level: i32,
    pub user_id: String,
    pub client_id: String,
    pub created_at: NaiveDateTime,
}

impl From<NewJsonCheckin> for NewCheckin {
    fn from(checkin: NewJsonCheckin) -> NewCheckin {
        NewCheckin {
            gps: PgPoint(checkin.gps[0], checkin.gps[1]),
            location_name: checkin.location_name,
            crowded_level: checkin.crowded_level,
            user_id: checkin.user_id,
            client_id: checkin.client_id,
            created_at: Utc::now().naive_utc(),
        }
    }
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
