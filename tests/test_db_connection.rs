use std::env;

use bukhgalter::db::{db, errors};

macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
  }

#[test]
fn test_db_init_invalid_url(){
    // setup environment variable for mongo
    let var = "MONGODB_URI".to_string();
    env::set_var(var.clone(), "invalid_url");
    match aw!(db::DB::init(var)) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true)
    };
}
  

#[test]
fn test_db_init_valid_url(){
    // setup environment variable for mongo
    let var = "MONGODB_URI_1".to_string();
    env::set_var(var.clone(), "mongodb://127.0.0.1:27017");
    match aw!(db::DB::init(var)) {
        Ok(_) => assert!(true),
        Err(e) => {println!("{} - {}", e, env::var("MONGODB_URI").unwrap()); assert!(false)}
    };
}