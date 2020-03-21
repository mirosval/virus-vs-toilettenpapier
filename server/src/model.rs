use crate::schema::checkins;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel_geography::types::GeogPoint;
use serde::{Deserialize, Serialize};

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
    pub gps: GeogPoint,
    pub location_name: String,
    pub crowded_level: i32,
    pub missing_goods: Vec<String>,
    pub user_id: String,
    pub client_id: String,
    pub created_at: NaiveDateTime,
}

impl From<NewJsonCheckin> for NewCheckin {
    fn from(checkin: NewJsonCheckin) -> NewCheckin {
        NewCheckin {
            gps: GeogPoint {
                x: checkin.gps[0],
                y: checkin.gps[1],
                srid: None,
            },
            location_name: checkin.location_name,
            crowded_level: checkin.crowded_level,
            missing_goods: checkin.missing_goods,
            user_id: checkin.user_id,
            client_id: checkin.client_id,
            created_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Queryable, QueryableByName, Serialize, Deserialize)]
#[table_name = "checkins"]
pub struct Checkin {
    pub id: i32,
    pub gps: GeogPoint,
    pub location_name: String,
    pub crowded_level: i32,
    pub missing_goods: Vec<String>,
    pub user_id: String,
    pub client_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckinsAroundRequest {
    pub gps: [f64; 2],
    pub radius: i32,
    pub offset: i32,
    pub limit: i32,
}
