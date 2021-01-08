extern crate serde_json;

use diesel::prelude::PgConnection;
use diesel::dsl::*;
use diesel::result::Error;
use diesel::*;
use crate::schema::{account, account::dsl::*};

#[derive(Insertable, AsChangeset)]
#[table_name="accounts"]
pub struct NewAccount{
    pub items: serde_json::Value,
    pub debtors: serde_json::Value,
    pub name: String,
    pub id: String
}