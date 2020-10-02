use mongodb::{Client, options::ClientOptions};
use std::env;
use std::error::Error;
use tokio;

mod db;

use db::{models, db::DB};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let db = DB::init().await?;

    // List the names of the databases in that deployment.
    for db_name in db.client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    Ok(())
}
