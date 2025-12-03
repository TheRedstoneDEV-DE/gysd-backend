use rocket::{routes, post, Route};
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json;
use crate::datatypes::{Login, Db};
use argon2::password_hash::{PasswordHasher, SaltString, rand_core::OsRng};
use argon2::Argon2;
use uuid::Uuid;
use rocket_db_pools::Connection;

// TODO: construct login + register, 
// hash passwords (-> argon2 crate), 
// store in database, 
// store token as cookie (separate uuid) 

#[post("/register", format="json", data="<data>") ]
pub async fn register(mut db: Connection<Db>, jar: &CookieJar<'_>, data: json::Json<Login>) -> Option<String> {
    // 1. Hash password 
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    // generate and salt the hash ('cause it doesn't taste good without salt)
    let hash = argon2.hash_password(data.password.as_bytes(), &salt).ok()?.to_string();

    sqlx::query("INSERT INTO users (uuid, name, password_hash) VALUES (?, ?, ?)")
        .bind(Uuid::new_v4().to_string())
        .bind(&data.username)
        .bind(&hash)
        .execute(&mut **db)
        .await
        .ok()?;
    


    Some("Success!".to_string())
}

pub fn routes() -> Vec<Route> {
    routes![register]
}
