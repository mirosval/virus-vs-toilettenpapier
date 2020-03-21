use super::Pool;
use crate::model::CheckinsAroundRequest;
use crate::model::{Checkin, NewCheckin, NewJsonCheckin};
use crate::schema::checkins;
use diesel::dsl::sql_query;
use diesel::RunQueryDsl;
use diesel_geography::sql_types::Geography;
use diesel_geography::types::GeogPoint;
use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn checkins_around(
    req: CheckinsAroundRequest,
    pool: Pool,
) -> Result<impl warp::Reply, Infallible> {
    pool.get()
        .and_then(|conn| {
            let v = GeogPoint {
                x: req.gps[0],
                y: req.gps[1],
                srid: None,
            };
            let res: Result<Vec<Checkin>, _> = sql_query(format!(
                "select * from checkins where ST_DWithin(gps, 'POINT({}, {})') ",
                1.1, 1.1
            ))
            .get_results::<Checkin>(&conn);
            Ok(warp::reply::with_status(
                warp::reply::json::<Vec<Checkin>>(&res.unwrap()),
                StatusCode::OK,
            ))
        })
        .or_else(|e| {
            error!("Failed listing checins {}", &e);
            Ok(warp::reply::with_status(
                warp::reply::json(&""),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        })
}

pub async fn list_checkins(pool: Pool) -> Result<impl warp::Reply, Infallible> {
    pool.get()
        .and_then(|conn| {
            use crate::schema::checkins::dsl::checkins;
            let res = checkins.load(&conn).unwrap();
            let checkin: Option<&Checkin> = res.first();
            Ok(warp::reply::with_status(
                warp::reply::json(&checkin),
                StatusCode::OK,
            ))
        })
        .or_else(|e| {
            error!("Failed listing checins {}", &e);
            Ok(warp::reply::with_status(
                warp::reply::json(&""),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        })
}

pub async fn create_checkin(
    json_checkin: NewJsonCheckin,
    pool: Pool,
) -> Result<impl warp::Reply, Infallible> {
    info!("create_checkin");
    let checkin = NewCheckin::from(json_checkin);
    pool.get()
        .and_then(|conn| {
            let res: Result<Checkin, _> = diesel::insert_into(checkins::table)
                .values(checkin)
                .get_result(&conn);
            match res {
                Ok(checkin) => {
                    info!("inserted checkin: {:?}", &checkin);
                    Ok(StatusCode::CREATED)
                }
                Err(e) => {
                    error!("error inserting checkin {}", &e);
                    Ok(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        })
        .or_else(|e| {
            error!("error inserting checkin {}", &e);
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        })
}