pub mod models;
pub mod handlers;
pub mod filters;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_as_jsonb;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
pub mod schema;