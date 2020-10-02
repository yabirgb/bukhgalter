use serde::{Serialize, Deserialize};
use mongodb::bson::{Document, document::ValueAccessError, Bson, doc};
use mongodb::{options::ClientOptions, Client, Collection};

mod errors;
use errors::Error::*;

const DB_NAME: &str = "IVDB";
const COLL: &str = "debtors";

const ID: &str = "_id";
const NAME: &str = "name";
const PAID_AMOUNT: &str = "paid_amount";
const FRACTION: &str = "fraction";
const PAID: &str = "paid";

#[derive(Serialize, Deserialize, Debug)]
struct Debtor{
    id: String,
    name: String,
    paid_amount: f64,
    fraction: f64,
    paid: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct Item{
    price: f32,
    date: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Account{
    items: Vec<Item>,
    debtors: Vec<Debtor>
}

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB{

    pub async fn init() -> Result<Self, errors::Error> {
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
        client_options.app_name = Some("booky".to_string());

        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }

    fn get_collection(&self) -> Collection {
        self.client.database(DB_NAME).collection(COLL)
    }

    pub async fn create_debtor(&self, entry: &Debtor) -> Result<(),errors::Error> {
        let doc = doc! {
            NAME: entry.name.clone(),
            PAID_AMOUNT: entry.paid_amount,
            PAID: entry.paid,
            FRACTION: entry.fraction,
        };

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
}

