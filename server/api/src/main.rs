#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

mod filters;
mod handlers;
mod model;
mod schema;

use diesel::r2d2;
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;
use warp::Filter;

type Pool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

fn get_connection_pool() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let cm = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(cm)
        .expect("build connection pool");
    pool
}

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "checkins=trace");
    }

    pretty_env_logger::init();
    let pool = get_connection_pool();
    let api = filters::checkins(pool);
    let cors = warp::cors()
        .allow_origin("http://localhost:5000")
        .allow_header("Content-Type")
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]);
    let routes = api.with(warp::log("checkins")).with(cors);

    info!("starting server");
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}

#[cfg(test)]
mod tests {
    use crate::get_connection_pool;
    use crate::model::CheckinsAroundRequest;
    use crate::model::NewJsonCheckin;
    use warp::http::StatusCode;
    use warp::test::request;

    use super::filters;

    fn make_checkin() -> NewJsonCheckin {
        NewJsonCheckin {
            gps: [53.55, 9.97],
            location_name: "some location".to_string(),
            crowded_level: 3,
            user_id: "some user".to_string(),
            client_id: "some client".to_string(),
            missing_goods: vec![String::from("flour")],
        }
    }

    #[tokio::test]
    async fn test_checkin() {
        let db = get_connection_pool();
        let api = filters::checkins(db);

        let res = request()
            .method("POST")
            .path("/v1/checkins")
            .json(&make_checkin())
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_checkins_around() {
        let db = get_connection_pool();
        let api = filters::checkins(db);

        let res = request()
            .method("POST")
            .path("/v1/checkins")
            .json(&make_checkin())
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::CREATED);

        let req = CheckinsAroundRequest {
            gps: [53.55, 9.97],
            radius: 1000,
            offset: 0,
            limit: 10,
        };

        let res = request()
            .method("POST")
            .path("/v1/checkins/around")
            .json(&req)
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::OK);
    }
}
