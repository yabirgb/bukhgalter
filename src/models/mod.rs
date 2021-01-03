use std::{sync::{Mutex,Arc}, env};
//use tokio::sync::Mutex;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use diesel::dsl::*;
use crate::schema::accounts as schema_accounts;

use serde::{Serialize};
use serde_json;

pub mod models;
pub mod errors;
pub mod requests;

// https://users.rust-lang.org/t/mutable-struct-fields-with-async-await/45395/7

pub trait DataManager: Send + Clone + 'static{
    fn new(&mut self);
    fn store(&self, acc: models::Account)->Result<(), errors::DataError>;
    fn get_by_id(&self, id: String) -> Result<models::Account, errors::DataError>;
    fn get_with_user(&self, user: String) -> Result<Vec<models::Account>, errors::DataError>;
    fn make_payment(&self, payment: &requests::Payment)->Result<models::Account, errors::AccountError>;
    fn update_account(&self, id:String, acc: requests::CreateAccount)->Result<models::Account, errors::DataError>;
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

    fn update_account(&self, id:String,  account: requests::CreateAccount)->Result<models::Account, errors::DataError>{
        
        let mut updated = false;
        let mut updated_account:Option<models::Account> = None;
        self.with_lock(|accounts|{
            for acc in accounts.iter_mut(){
                if acc.id == id{
                    acc.debtors = account.debtors;
                    acc.items = account.items;
                    acc.name = account.name;
                    updated = true;
                    updated_account = Some(acc.clone());
                    break;
                }
            }
        });

        if updated{
            Ok(updated_account.unwrap())
        }else{
            Err(errors::DataError::NotFound)
        }
        
    }
}

pub fn blank_db() -> MemoryDataManager{
    MemoryDataManager{
        db: Arc::new(Mutex::new(Vec::new()))
    }   
}

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;
type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn pg_pool(database_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("Postgres connection pool could not be created")
}

#[derive(Clone)]
pub struct PGDataManager{
    pub db: PgPool
}

impl PGDataManager{
    fn get_con(&self)->Result<PooledPg, errors::DataManagerError>{
        match self.db.get() {
            Ok(conn) => Ok(conn),
            Err(_) => Err(errors::DataManagerError::ConnectionError)
        }
    }
}

impl DataManager for PGDataManager{
    fn new(&mut self){
        self.db = pg_pool(&env::var("DATABASE_URL").unwrap())
    }

    fn store(&self, account: models::Account)->Result<(), errors::DataError>{
        match self.get_con(){
            Ok(conn) => {
                insert_into(schema_accounts::dsl::accounts)
                .values((
                    schema_accounts::dsl::items.eq(account.items),
                    schema_accounts::dsl::debtors.eq(account.debtors),
                    schema_accounts::dsl::name_.eq(account.name),
                    schema_accounts::dsl::id.eq(account.id)
                )).execute(&conn);
            }
            Err(_)=> return Err(errors::DataError::NotFound)
        };
        
        Ok(())
    }

    fn get_by_id(&self, id: String)->Result<models::Account, errors::DataError>{
        
        match self.get_con(){
            Ok(conn) => {
                match schema_accounts::dsl::accounts.find(id).first(&conn) {
                    Ok(acc) => return Ok(acc),
                    Err(_) => {println!("Account not found"); return Err(errors::DataError::NotFound)}
                }
            }
            Err(_)=> {println!("Error getting connection");return Err(errors::DataError::NotFound)}
        };
    }

    fn get_with_user(&self, user: String)->Result<Vec<models::Account>, errors::DataError>{
        let mut results: Vec<models::Account> = Vec::new();

        match self.get_con(){
            Ok(conn) => {
                match schema_accounts::dsl::accounts.filter(
                            sql("debtors->>'name'? 'Gustavo'")
                        ).load::<models::Account>(&conn) 
                    {
                    Ok(acc) => {println!("Encontrado");return Ok(acc)},
                    Err(e) => {println!("{:?}", e); return Err(errors::DataError::NotFound)}
                }
            }
            Err(_)=> {println!("Error getting connection");return Err(errors::DataError::NotFound)}
        };

        Ok(results)
        //return Err(errors::DataError::NotFound)
    }

    fn make_payment(&self, payment: &requests::Payment)->Result<models::Account, errors::AccountError>{
        
        let mut account: Option<models::Account> = None;
/*
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
*/
        match account{
            Some(x)=>Ok(x),
            None =>Err(errors::AccountError::DebtorNotFound)
        }
    }

    fn update_account(&self, id:String,  account: requests::CreateAccount)->Result<models::Account, errors::DataError>{
        
        let mut updated = false;
        let mut updated_account:Option<models::Account> = None;
/*        self.with_lock(|accounts|{
            for acc in accounts.iter_mut(){
                if acc.id == id{
                    acc.debtors = account.debtors;
                    acc.items = account.items;
                    acc.name = account.name;
                    updated = true;
                    updated_account = Some(acc.clone());
                    break;
                }
            }
        });
*/
        if updated{
            Ok(updated_account.unwrap())
        }else{
            Err(errors::DataError::NotFound)
        }
        
    }
}