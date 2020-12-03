use std::sync::{Mutex,Arc};
use std::convert::Infallible;
//use tokio::sync::Mutex;
use async_trait::async_trait;


pub mod models;
pub mod errors;

// https://users.rust-lang.org/t/mutable-struct-fields-with-async-await/45395/7

pub trait DataManager: Send + Clone{
    fn new(&mut self);
    fn store(&self, acc: models::Account)->Result<(), errors::DataError>;
    fn get_by_id(&self, id: String) -> Result<models::Account, errors::DataError>;
    //fn clone(&self)->Self;
}

pub type MemoryDb = Arc<Mutex<Vec<models::Account>>>;

#[derive(Debug, Clone)]
pub struct MemoryDataManager{
    db: MemoryDb
}


impl MemoryDataManager{
    fn with_lock<F, T>(&self, func: F) -> T
    where
        F: FnOnce(&mut std::vec::Vec<models::Account>) -> T,
    {
        let mut lock = self.db.lock().unwrap();
        let result = func(&mut *lock);
        drop(lock);
        result
    }
}

impl DataManager for MemoryDataManager{
    fn new(&mut self){
        self.db = Arc::new(Mutex::new(Vec::new()))
    }

    fn store(&self, account: models::Account)->Result<(), errors::DataError>{
        self.with_lock(|accounts|{
            accounts.push(account.clone());
        });
        
        Ok(())
    }

    fn get_by_id(&self, id: String)->Result<models::Account, errors::DataError>{
        
        self.with_lock(|accounts|{
            let account: Option<&models::Account> = accounts.iter().find(|a| a.id == id);
            match account{
                Some(acc)=>Ok(acc.clone()),
                None=>Err(errors::DataError::NotFound)
            }
        })
        


    }
}

pub fn blank_db() -> MemoryDataManager{
    MemoryDataManager{
        db: Arc::new(Mutex::new(Vec::new()))
    }   
}