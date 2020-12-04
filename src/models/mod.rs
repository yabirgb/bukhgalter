use std::sync::{Mutex,Arc};
//use tokio::sync::Mutex;

pub mod models;
pub mod errors;
pub mod requests;

// https://users.rust-lang.org/t/mutable-struct-fields-with-async-await/45395/7

pub trait DataManager: Send + Clone{
    fn new(&mut self);
    fn store(&self, acc: models::Account)->Result<(), errors::DataError>;
    fn get_by_id(&self, id: String) -> Result<models::Account, errors::DataError>;
    fn get_with_user(&self, user: String) -> Result<Vec<models::Account>, errors::DataError>;
    fn make_payment(&self, payment: &requests::Payment)->Result<models::Account, errors::AccountError>;
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

    fn get_with_user(&self, user: String)->Result<Vec<models::Account>, errors::DataError>{
        let mut results: Vec<models::Account> = Vec::new();
        self.with_lock(|accounts|{
            for acc in accounts.iter(){
                for debtor in acc.debtors.iter(){
                    if debtor.name == user{
                        results.push(acc.clone());
                    }
                }
            }
            
            
        });

        Ok(results)
        //return Err(errors::DataError::NotFound)
    }

    fn make_payment(&self, payment: &requests::Payment)->Result<models::Account, errors::AccountError>{
        
        let mut account: Option<models::Account> = None;

        self.with_lock(|accounts|{
            for acc in accounts.iter_mut(){
                if acc.id == payment.account_id{
                    match acc.pay_by_debtor(payment.debtor.clone(), payment.amount.into()){
                        Ok(_)  => {account = Some(acc.clone())},
                        Err(_e) => {}
                    }
                }
            }
        });

        match account{
            Some(x)=>Ok(x),
            None =>Err(errors::AccountError::DebtorNotFound)
        }

        
    }
}

pub fn blank_db() -> MemoryDataManager{
    MemoryDataManager{
        db: Arc::new(Mutex::new(Vec::new()))
    }   
}