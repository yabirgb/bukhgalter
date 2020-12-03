use std::sync::Arc;
use std::convert::Infallible;
use tokio::sync::Mutex;
use async_trait::async_trait;


pub mod models;
pub mod errors;

#[async_trait]
pub trait DataManager {
    fn new(&mut self);
    async fn store(&self, acc: models::Account)->Result<(), errors::DataError>;
    async fn get_by_id(&self, id: String) -> Result<models::Account, errors::DataError>;
    async fn clone(&self)->Self;
}

pub type MemoryDb = Arc<Mutex<Vec<models::Account>>>;

struct MemoryDataManager{
    db: MemoryDb
}

#[async_trait]
impl DataManager for MemoryDataManager{
    fn new(&mut self){
        self.db = Arc::new(Mutex::new(Vec::new()))
    }

    async fn store(&self, account: models::Account)->Result<(), errors::DataError>{
        let mut accounts = self.db.lock().await;
        accounts.push(account.clone());
        Ok(())
    }

    async fn get_by_id(&self, id: String)->Result<models::Account, errors::DataError>{
        let accounts = self.db.lock().await;
        let account: Option<&models::Account> = accounts.iter().find(|a| a.id == id);

        match account{
            Some(acc)=>Ok(acc.clone()),
            None=>Err(errors::DataError::NotFound)
        }
    }

    async fn clone(&self)->Self{
        MemoryDataManager{db: self.db.clone()}
    }
}

pub fn blank_db() -> MemoryDb{
    Arc::new(Mutex::new(Vec::new()))
}