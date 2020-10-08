use serde::{Serialize, Deserialize};

use mongodb::bson::{Document, doc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Debtor{
    pub id: String,
    pub name: String,
    pub paid_amount: f64,
    pub fraction: f64,
    pub paid: bool
}

// consts in the documents
pub const ID: &str = "_id";
pub const NAME: &str = "name";
pub const PAID_AMOUNT: &str = "paid_amount";
pub const FRACTION: &str = "fraction";
pub const PAID: &str = "paid";


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item{
    pub price: f32,
    pub date: u32,
    pub name: String
}

pub const PRICE: &str = "price";
pub const DATE: &str = "date";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account{
    pub items: Vec<Item>,
    pub debtors: Vec<Debtor>
}

pub const ITEMS: &str = "items";
pub const DEBTORS: &str = "debtors";

impl Debtor{
    pub fn to_doc(&self) -> Document{
        doc! {
            NAME: self.name.clone(),
            PAID_AMOUNT: self.paid_amount,
            PAID: self.paid,
            FRACTION: self.fraction,
        }
    }

    pub fn to_debtor(doc: &Document){}

    pub fn update_paid_amount(&mut self, new_paid_amount:f64){
        self.paid_amount = new_paid_amount;
    }

    pub fn rename_debtor(&mut self, new_name: String){
        self.name = new_name.clone();
    }

    pub fn set_fraction(&mut self, new_fraction: f64){
        self.fraction = new_fraction;
    }

    pub fn toggle_paid(&mut self){
        self.paid = !self.paid;
    }
}

impl Item{
    pub fn to_doc(&self) -> Document{
        doc! {
            PRICE: self.price,
            DATE: self.date,
            NAME: self.name.clone()
        }
    }

    pub fn to_item(doc: &Document){}
}

impl Account{
    pub fn to_doc(&self) -> Document{
        doc!{
            ITEMS: self.items.clone().into_iter().map(|x| x.to_doc()).collect::<Vec<_>>(),
            DEBTORS: self.debtors.clone().into_iter().map(|x| x.to_doc()).collect::<Vec<_>>()
        }
    }

    pub fn to_account(doc: &Document){}
}