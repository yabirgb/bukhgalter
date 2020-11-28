use std::sync::Arc;
use tokio::sync::Mutex;

pub mod models;
pub mod errors;

pub type Db = Arc<Mutex<Vec<models::Account>>>;

pub fn blank_db() -> Db{
    Arc::new(Mutex::new(Vec::new()))
}