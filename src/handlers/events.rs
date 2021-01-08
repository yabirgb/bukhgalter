use warp::{http::StatusCode, Filter};
use warp::http::header::{HeaderMap, HeaderValue};
use serde::{Serialize};
use serde_json::json;
use crate::models::models::{Account};
use std::convert::Infallible;
use crate::models::{DataManager};
use crate::models::requests::{Payment, CreateAccount};

extern crate rand;

use rand::Rng; 
use rand::distributions::Alphanumeric;

#[derive(Serialize, Debug, Clone)]
pub struct CustomError{
    pub error: String
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
                StatusCode::OK
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

    let uri = format!("/events/{}", acc.id);

    let mut headers = HeaderMap::new();
    headers.insert("Location", HeaderValue::from_str(&uri).unwrap());

    let mut reply;// = warp::reply::with_status(warp::reply::json(&acc), StatusCode::CREATED);
    let builder = warp::http::response::Builder::new();

    reply = builder
    .header("content-type", "application/json")
    .header("Location", uri)
    .status(StatusCode::CREATED)
    .body(json!(&acc).to_string())
    .unwrap();
    //.and(warp::reply::json(&acc)
    Ok(reply)
}

pub async fn update_event(acc: CreateAccount, db: impl DataManager) -> Result<impl warp::Reply, Infallible>{

    let id = acc.id.clone();
    match db.update_account(id, acc.clone()){
        Ok(a)=> Ok(warp::reply::with_status(warp::reply::json(&a), StatusCode::ACCEPTED)),
        Err(e) => Ok(warp::reply::with_status(warp::reply::json(&e), StatusCode::NOT_FOUND)),
    }
}



pub async fn make_payment(payment: Payment, db: impl DataManager) -> Result<impl warp::Reply, Infallible>{

    let acc = db.make_payment(&payment);

    match acc{
        Ok(a) => return Ok(warp::reply::with_status(warp::reply::json(&a), StatusCode::OK)),
        Err(e) => return Ok(warp::reply::with_status(warp::reply::json(&e), StatusCode::NOT_FOUND))
    }
    
}