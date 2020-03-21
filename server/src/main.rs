#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

pub mod model;
pub mod schema;

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
    let routes = api.with(warp::log("checkins"));

    info!("starting server");
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}

mod filters {
    use super::handlers;
    use super::model::NewCheckin;
    use super::Pool;
    use warp::Filter;

    pub fn checkins(
        db: Pool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("v1").and(checkins_list(db.clone()).or(checkins_create(db.clone())))
    }

    pub fn checkins_list(
        db: Pool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("checkins").and(warp::get()).map(|| "checkin")
    }

    pub fn checkins_create(
        db: Pool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("checkins")
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

    fn with_db(
        db: Pool,
    ) -> impl Filter<Extract = (Pool,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}

mod handlers {
    use super::Pool;
    use crate::model::{Checkin, NewCheckin};
    use crate::schema::checkins;
    use diesel::RunQueryDsl;
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn create_checkin(
        checkin: NewCheckin,
        pool: Pool,
    ) -> Result<impl warp::Reply, Infallible> {
        info!("Handler");
        dbg!("aaa");
        pool.get()
            .and_then(|conn| {
                let res: Result<Checkin, _> = diesel::insert_into(checkins::table)
                    .values(checkin)
                    .get_result(&conn);
                match res {
                    Ok(checkin) => Ok(StatusCode::CREATED),
                    Err(e) => {
                        // log
                        Ok(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            })
            .or_else(|e| {
                // log
                Ok(StatusCode::INTERNAL_SERVER_ERROR)
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::get_connection_pool;
    use crate::model::NewCheckin;
    use chrono::Utc;
    use diesel_geometry::data_types::PgPoint;
    use warp::http::StatusCode;
    use warp::test::request;

    use super::filters;

    #[tokio::test]
    async fn test_checkin() {
        let db = get_connection_pool();
        let api = filters::checkins(db);

        let res = request()
            .method("POST")
            .path("/v1/checkins")
            .json(&NewCheckin {
                gps: PgPoint(1.1, 2.2),
                location_name: "some location".to_string(),
                crowded_level: 3,
                user_id: "some user".to_string(),
                client_id: "some client".to_string(),
                created_at: Utc::now().naive_utc(),
            })
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::CREATED);
    }
}
