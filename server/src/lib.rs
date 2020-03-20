#[macro_use]
extern crate diesel;
extern crate dotenv;

mod schema;
mod model;

use model::*;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub async fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub async fn create_location<'a>(conn: &PgConnection, name: &'a str) -> QueryResult<Location> {
    use schema::location;
    let new_location = NewLocation {
        name: name
    };

    diesel::insert_into(location::table)
        .values(&new_location)
        .get_result(conn)
}