use super::models::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateAccount{
    pub items: Vec<Item>,
    pub debtors: Vec<Debtor>,
    pub name: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateAccount{
    pub items: Vec<Item>,
    pub debtors: Vec<Debtor>,
    pub name: String,
    pub id: String
}

#[derive(Serialize,Deserialize, Debug, Clone)]
pub struct Payment{
    pub debtor: String,
    pub account_id: String,
    pub amount: f32
}
