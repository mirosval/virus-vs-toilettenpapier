use super::handlers;
use super::model::NewJsonCheckin;
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
    warp::path("checkins")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::list_checkins)
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

fn json_body() -> impl Filter<Extract = (NewJsonCheckin,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn with_db(db: Pool) -> impl Filter<Extract = (Pool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
