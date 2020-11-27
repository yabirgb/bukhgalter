//use mongodb::{Client, options::ClientOptions};
use std::env;
use std::error::Error;
use warp::http::StatusCode;
use warp::Filter;
use serde::Serialize;
//use tokio;

mod models;

#[derive(Debug, Serialize)]
struct Health{
    status: String
}

#[tokio::main]
async fn main(){

    let health_route = warp::path("status")
        .map(|| warp::reply::json(&Health{status:"OK".to_string()}));

    let routes = health_route
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
