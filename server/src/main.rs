use warp::Filter;

#[tokio::main]
async fn main() {
    let _ = establish_connection().await;
    let checkin = warp::path!("v1" / "checkin").map(|| "checkin");
    let app = checkin;

    warp::serve(app)
        .run(([127, 0, 0, 1], 3000)).await;
}
