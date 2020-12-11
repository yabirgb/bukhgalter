use warp::{http::StatusCode, Filter};
use crate::models::models::{Account};
use std::convert::Infallible;
use crate::models::{DataManager};
use crate::models::requests::{Payment, CreateAccount};

use crate::handlers::events::{make_payment, create_event, event_info, user_events, update_event};

const BODY_SIZE:u64=1024*16;


fn with_db(db: impl DataManager) -> impl Filter<Extract = (impl DataManager,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}


pub fn events_endpoint(
    db: impl DataManager,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    event_make_payment(db.clone())
        .or(event_create(db.clone()))
        .or(event_get(db.clone()))
        .or(event_get_by_user(db.clone()))
}

/// PATCH /events/pay with JSON body
pub fn event_make_payment(
    db: impl DataManager,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "v1" / "events" / "pay")
        .and(warp::patch())
        .and(warp::body::content_length_limit(BODY_SIZE))
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(make_payment)
}

/// POST /events/create with JSON body
pub fn event_create(
    db: impl DataManager,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "v1" / "events")
        .and(warp::post())
        .and(warp::body::content_length_limit(BODY_SIZE))
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(create_event)
}

/// GET /events/{events_id}
pub fn event_get(
    db: impl DataManager,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "v1" / "events" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(event_info)
}

pub fn event_get_by_user(
    db: impl DataManager,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "v1" /  "users" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(user_events)
}



pub fn event_update(
    db: impl DataManager,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "v1" / "events" / String)
        .and(warp::put())
        .and(warp::body::content_length_limit(BODY_SIZE))
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(update_event)
}