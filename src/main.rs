//use mongodb::{Client, options::ClientOptions};

use std::{env, fmt, error};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use warp::{Filter, http::Response};
use etcd_client::{Client, Error};
//extern crate pretty_env_logger;
extern crate syslog;
use syslog::{Facility, Formatter3164, BasicLogger};
#[macro_use] 
extern crate log;
use log::{SetLoggerError, LevelFilter};


#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_as_jsonb;

//use tokio;
pub mod models;
pub mod handlers;
pub mod filters;
pub mod schema;
//#[derive(Debug, Serialize)]
//struct Health{
//    status: String
//}



use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum EnvRead {
    #[error("Variable not found")]
    VariableNotFound,
    #[error("Error reading variable from etcd")]
    etcdReadError,
}


async fn get_env_var(var:String)->Result<String, EnvRead>{

    // etcd client
    let etcd_host = env::var("ETCD_SERVER").unwrap_or_else(|_|"localhost".to_string().to_string());
    let etcd_port = env::var("ETCD_PORT").unwrap_or_else(|_|"2379".to_string());
    let etcd_connection = format!("{}:{}", etcd_host, etcd_port);



    let client = Client::connect([etcd_connection], None).await;

    // use etcd. If key not found look in env. If not found in env return Not found
    match client{
        Ok(mut c) => {
            match c.get(var.clone(), None).await{
                Ok(s) =>{
                    match s.kvs().first(){
                        Some(value) => return Ok(value.value_str().unwrap().to_string()),
                        None =>  {
                            match env::var(var){
                                Ok(s)=> return Ok(s),
                                Err(_)=> return Err(EnvRead::VariableNotFound)
                            }
                        }
                    }
                }
                Err(_) => return Err(EnvRead::etcdReadError)
            }
        }
        Err(_) => {
            match env::var(var){
                Ok(s)=> return Ok(s),
                Err(_)=> return Err(EnvRead::VariableNotFound)
            }
        }
    }
}

async fn get_env_var_or_default(var: String, default: String)->String{
    match get_env_var(var).await {
        Ok(v)=> v,
        Err(_) => default
    }
}

#[tokio::main]
async fn main(){
    pretty_env_logger::init();

    match &*    (get_env_var_or_default("mode".to_string(), "dev".to_string()).await){
        "prod" => {
            let formatter = Formatter3164 {
                facility: Facility::LOG_SYSLOG,
                hostname: None,
                process: "bukhgalter".into(),
                pid: 0,
            };

            let log_host = get_env_var("log_host".to_string()).await.unwrap();
            let log_port = get_env_var("log_port".to_string()).await.unwrap();

            //syslog::init(Facility::LOG_USER, LevelFilter::Debug, Some("bukhgalter"));
            match syslog::tcp(formatter, format!("{}:{}", log_host, log_port)){
                Ok(logger) => {
                    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
                    .map(|()| log::set_max_level(LevelFilter::Debug));    
                    println!("Set logger");
                    info!("log set");
                }
                Err(_) => println!("Error connecting to log service")
            };
            //syslog::init_tcp("logs.papertrailapp.com:10482", "berlin".to_string(), Facility::LOG_SYSLOG, LevelFilter::Info);

        }
        _ =>{}
    }

    //let health_route = warp::path("status")
    //    .map(|| warp::reply::json(&Health{status:"OK".to_string()}));

    let health_route = warp::path("status")
        .map(|| Response::builder()
            .header("Content-Type", "application/json")
            .body(r#"{"status": "OK"}"#)
    );

    let db = models::PGDataManager{db: models::pg_pool(&env::var("DATABASE_URL").unwrap())};

    let api_events = filters::events::events_endpoint(db);

    //println!("TEST {:?}", get_env_var("adios".to_string()).await);

    let host = get_env_var_or_default("HOST".to_string(), "127.0.0.1".to_string()).await;
    let port = get_env_var_or_default("PORT".to_string(), "8000".to_string()).await;

    let routes = health_route
        .or(api_events)
        .with(warp::cors().allow_any_origin())
        .with(warp::log("events"));


    println!("Server running at {}:{}", host, port);
    warp::serve(routes).run((host.parse::<IpAddr>().unwrap(), port.parse().unwrap())).await;
}