#[macro_use]
extern crate diesel;

pub mod model;
pub mod schema;

use std::sync::Arc;
use warp::Filter;
use dotenv::dotenv;
use diesel::PgConnection;
use std::env;
use diesel::connection::Connection;
use tokio::sync::Mutex;

pub type Db = Arc<Mutex<PgConnection>>;

pub async fn establish_connection() -> Db {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pgc = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    Arc::new(Mutex::new(pgc))
}

#[tokio::main]
async fn main() {
    let db = establish_connection().await;
    let api = filters::checkins(db);
let routes = api.with(warp::log("checkins"));

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3000)).await;
}

mod filters {
    use warp::Filter;
    use super::handlers;
    use super::model::NewCheckin;
    use super::Db;

    pub fn checkins(
        db: Db
    ) -> impl Filter<Extract = impl warp::Reply, Error= warp::Rejection> + Clone {
        checkins_list(db)
    }

    pub fn checkins_list(
        db: Db 
    ) -> impl Filter<Extract = impl warp::Reply, Error= warp::Rejection> + Clone {
        warp::path!("v1" / "checkins").map(|| "checkin")
    }

    pub fn checkins_create(
        db: Db 
    ) -> impl Filter<Extract = impl warp::Reply, Error= warp::Rejection> + Clone {
        warp::path!("v1" / "checkins")
            .and(warp::post())
            .and(json_body())
            .and(with_db(db))
            .and_then(handlers::create_checkin)
    }

    fn json_body() -> impl Filter<Extract = (NewCheckin,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}

mod handlers {
    use crate::Db;
use crate::model::NewCheckin;
use std::convert::Infallible;
    use warp::http::StatusCode;
    
    pub async fn create_checkin(create: NewCheckin, db: Db) -> Result<impl warp::Reply, Infallible> {
        Ok(StatusCode::CREATED)
    }

    pub async fn get_markets(lat: f64, lon: f64, db: Db) -> Result<impl warp::Reply, Infallible> {
        Ok(StatusCode::ACCEPTED)
    }
}
