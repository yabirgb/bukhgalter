use thiserror::Error;
use serde::{Serialize};
/*
#[derive(Error, Debug)]
pub enum Error {
    #[error("mongodb error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    #[error("error during mongodb query: {0}")]
    MongoQueryError(mongodb::error::Error),
    #[error("could not access field in document: {0}")]
    MongoDataError(#[from] bson::document::ValueAccessError),
    #[error("invalid id used: {0}")]
    InvalidIDError(String),
}
*/

#[derive(Error, Debug, PartialEq, Serialize)]
pub enum AccountError {
    #[error("User not found in the debtors' list")]
    DebtorNotFound,
    #[error("Proportions don't sum one")]
    InvalidProportions,
}

#[derive(Error, Debug, PartialEq, Serialize)]
pub enum DataError {
    #[error("Error storing entry")]
    InsertError,
    #[error("Object not found")]
    NotFound,
    #[error("I never fail")]
    Infallible,
}