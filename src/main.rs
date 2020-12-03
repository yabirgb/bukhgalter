//use mongodb::{Client, options::ClientOptions};
use warp::{Filter, http::Response};
//use tokio;

pub mod models;
pub mod handlers;
//#[derive(Debug, Serialize)]
//struct Health{
//    status: String
//}

#[tokio::main]
async fn main(){

    //let health_route = warp::path("status")
    //    .map(|| warp::reply::json(&Health{status:"OK".to_string()}));

    let health_route = warp::path("status")
        .map(|| Response::builder()
            .header("Content-Type", "application/json")
            .body(r#"{"status": "OK"}"#)
    );

    let db = models::blank_db();

    let api = handlers::events::events_endpoint(db);

    let routes = health_route
        .or(api)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
