use warp::Filter;
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {
    let _ = establish_connection().await;
    let api = filters::checkins();
let routes = api.with(warp::log("checkins"));

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3000)).await;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Checkin;

mod filters {
    use super::Checkin;
    use warp::Filter;
    use super::handlers;

    pub fn checkins(
        // db: Db
    ) -> impl Filter<Extract = impl warp::Reply, Error= warp::Rejection> + Clone {
        checkins_list()
    }

    pub fn checkins_list(
        // db: Db 
    ) -> impl Filter<Extract = impl warp::Reply, Error= warp::Rejection> + Clone {
        warp::path!("v1" / "checkins").map(|| "checkin")
    }

    pub fn checkins_create(
        // db: Db 
    ) -> impl Filter<Extract = impl warp::Reply, Error= warp::Rejection> + Clone {
        warp::path!("v1" / "checkins").and(warp::post()).and(json_body()).and_then(handlers::create_checkin)
    }

    fn json_body() -> impl Filter<Extract = (Checkin,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

    // fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    //     warp::any().map(move || db.clone())
    // }
}

mod handlers {
    use std::convert::Infallible;
    use warp::http::StatusCode;
    use super::Checkin;
    
    pub async fn create_checkin(create: Checkin/*, db: Db*/) -> Result<impl warp::Reply, Infallible> {
        Ok(StatusCode::CREATED)
    }
}
