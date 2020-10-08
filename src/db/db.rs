use mongodb::bson::{Document, document::ValueAccessError, Bson, doc};
use mongodb::{options::ClientOptions, Client, Collection};

use super::errors::Error::*;
use super::errors;
use super::models::{Item, Debtor, ID, NAME, PAID_AMOUNT, FRACTION, PAID};

const DB_NAME: &str = "IVDB";
const COLL: &str = "debtors";

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB{

    pub async fn init(var: String) -> Result<Self, errors::Error> {
        let uri = std::env::var(var).expect("no URI given!");

        let mut client_options:ClientOptions = ClientOptions::parse(&uri).await?;
        client_options.app_name = Some("bukhgalter".to_string());

        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }

    fn get_collection(&self) -> Collection {
        self.client.database(DB_NAME).collection(COLL)
    }

    pub async fn create_debtor(&self, entry: &Debtor) -> Result<(),errors::Error> {
        
        let doc = entry.to_doc();

        self.get_collection()
            .insert_one(doc, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }


    fn doc_to_debtor(&self, doc: &Document) -> Result<Debtor, ValueAccessError> {
        let id = doc.get_object_id(ID)?;
        let name = doc.get_str(NAME)?;
        let fraction = doc.get_f64(FRACTION)?;
        let paid = doc.get_bool(PAID)?;
        let paid_amount = doc.get_f64(PAID_AMOUNT)?;
    
        let debtor = Debtor {
            id: id.to_hex(),
            name: name.to_owned(),
            fraction: fraction,
            paid_amount:paid_amount,
            paid: paid
        };
        Ok(debtor)
    }

    // Pay a whole item in teh account
    fn pay_item(&self, item_id: u32){}

    // Add a debtor to an account
    fn add_debtor(&self, account_id: u32){}

    // Pay a fraction of the account
    fn pay_fraction(&self, account_id: u32, fraction: f32){}

    // Add an item to an account
    fn add_item_to_account(&self, account_id: u32, price:f32){}

    // Add debtor to an account
    fn create_account(&self, items: Vec<Item>, debtors:Vec<Debtor>){}
}

