use super::handlers;
use super::model::{CheckinsAroundRequest, NewJsonCheckin};
use super::Pool;
use serde::de::DeserializeOwned;
use warp::Filter;

pub fn checkins(
    db: Pool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("v1").and(
        checkins_around(db.clone())
            .or(checkins_around_cors())
            .or(checkins_list(db.clone()))
            .or(checkins_create(db.clone())),
    )
}

fn checkins_around_cors() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("checkins" / "around")
        .and(warp::head())
        .and_then(|| {
            Ok(warp::reply::with_header(
                warp::reply::json(&""),
                "Access-Control-Allow-Origin",
                "http://localhost:5000",
            ))
        })
}

fn checkins_around(
    db: Pool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("checkins" / "around")
        .and(warp::post())
        .and(json_body::<CheckinsAroundRequest>())
        .and(with_db(db))
        .and_then(handlers::checkins_around)
}

fn checkins_list(
    db: Pool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("checkins")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::list_checkins)
}

fn checkins_create(
    db: Pool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("checkins")
        .and(warp::post())
        .and(json_body::<NewJsonCheckin>())
        .and(with_db(db))
        .and_then(handlers::create_checkin)
}

fn json_body<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
where
    T: DeserializeOwned + Send,
{
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn with_db(db: Pool) -> impl Filter<Extract = (Pool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
