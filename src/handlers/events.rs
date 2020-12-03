use warp::{http::StatusCode, Filter};
use serde::{Serialize, Deserialize};
use crate::models::models::{Account, Debtor, Item};
use std::convert::Infallible;
use crate::models::{DataManager};

extern crate rand;

use rand::Rng; 
use rand::distributions::Alphanumeric;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateAccount{
    pub items: Vec<Item>,
    pub debtors: Vec<Debtor>,
    pub name: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct CustomError{
    pub error: String
}

fn with_db(db: impl DataManager) -> impl Filter<Extract = (impl DataManager,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (CreateAccount,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn events_endpoint(
    db: impl DataManager,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        event_create(db.clone())
        .or(get_event(db.clone()))
        .or(get_by_user(db.clone()))
}

/// POST /events/create with JSON body
pub fn event_create(
    db: impl DataManager,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("events" / "create")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(create_event)
}

/// GET /events/{events_id}
pub fn get_event(
    db: impl DataManager,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("events" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(event_info)
}

pub fn get_by_user(
    db: impl DataManager,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(user_events)
}

pub async fn event_info(id: String, db: impl DataManager) -> Result<impl warp::Reply, Infallible>{

    let account = db.get_by_id(id);

    match account{
        Ok(acc) => Ok(            
            warp::reply::with_status(
                warp::reply::json(&acc),
                StatusCode::CREATED
            )
        ),
        Err(_e) =>  Ok(
            warp::reply::with_status(
                warp::reply::json(&CustomError{error:"Evento no encontrado".to_string()}),
                StatusCode::NOT_FOUND
            )
        )
    }
}

pub async fn user_events(username: String, db: impl DataManager) -> Result<impl warp::Reply, Infallible>{

    let accounts = db.get_with_user(username);

    match accounts{
        Ok(acc) => Ok(            
            warp::reply::with_status(
                warp::reply::json(&acc),
                StatusCode::CREATED
            )
        ),
        Err(_e) =>  Ok(
            warp::reply::with_status(
                warp::reply::json(&CustomError{error:"Usuario sin eventos".to_string()}),
                StatusCode::NOT_FOUND
            )
        )
    }
}


pub async fn create_event(create: CreateAccount, db: impl DataManager) -> Result<impl warp::Reply, Infallible>{

    let id = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect::<String>(); 

    let acc: Account = Account{
        items: create.items,
        debtors: create.debtors,
        name: create.name,
        id
    };

    db.store(acc.clone());

    //.and(warp::reply::json(&acc)
    Ok(warp::reply::with_status(warp::reply::json(&acc), StatusCode::CREATED))
}