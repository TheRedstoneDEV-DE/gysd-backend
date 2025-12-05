use rocket::{routes, get, post};
use rocket::serde::json;
use rocket::http::{CookieJar, Status};
use crate::datatypes::{Quick, Db};
use crate::helpers;
use rocket_db_pools::Connection;

//pub async fn get_quick(mut db: Connection<Db>)
