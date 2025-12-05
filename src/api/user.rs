use rocket::{routes, post, get, Route};
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::serde::json;
use crate::datatypes::{Login, Db, UserData};
use argon2::password_hash::{PasswordHasher, SaltString, rand_core::OsRng};
use argon2::Argon2;
use uuid::Uuid;
use rocket_db_pools::Connection;
use crate::helpers;
use time::Duration;

// TODO: construct login + register, 
// hash passwords (-> argon2 crate), 
// store in database, 
// store token as cookie (separate uuid) 

#[post("/register", format="json", data="<data>") ]
pub async fn register(mut db: Connection<Db>, jar: &CookieJar<'_>, data: json::Json<Login>) -> Option<Status> {
    if jar.get("auth").is_some() {
        return Some(Status::Conflict);
    }
    // 1. Hash password 
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    // generate and salt the hash ('cause they don't taste good without salt)
    let hash = argon2.hash_password(data.password.as_bytes(), &salt).ok()?.to_string();

    let uuid = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO users (uuid, name, password_hash) VALUES (?, ?, ?)")
        .bind(&uuid)
        .bind(&data.username)
        .bind(&hash)
        .execute(&mut **db)
        .await
        .ok()?;
    
    jar.add(Cookie::build(("auth", uuid))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Lax)
            .max_age(Duration::days(30))
    );

    Some(Status::Created)
}

#[post("/login", format="json", data="<data>")]
pub async fn login(db: Connection<Db>, jar: &CookieJar<'_>, data: json::Json<Login>) -> Option<Status> {
    if jar.get("auth").is_some() {
        return Some(Status::Ok)
    }
    if let Some((valid, uuid)) = helpers::validate_auth(data, db).await {
        if valid {
            jar.add(Cookie::build(("auth", uuid))
                .http_only(true)
                .secure(true)
                .max_age(Duration::days(30))
                .same_site(SameSite::Lax)
            );
            return Some(Status::Ok)
        }
        return Some(Status::Forbidden)
    }
    return Some(Status::InternalServerError)
}

#[get("/get_data")]
pub async fn get_data(mut db: Connection<Db>, jar: &CookieJar<'_>) -> Option<json::Json<UserData>> {
    let auth_uuid = jar.get("auth")?.value().to_string();
    let missions = helpers::get_missions(auth_uuid.clone(), &mut db).await;
    let quicks = helpers::get_quicks(auth_uuid.clone(), &mut db).await;
    let habits = helpers::get_habits(auth_uuid, &mut db).await;

    Some(rocket::serde::json::Json(UserData{
        missions: missions,
        quicks: quicks,
        habits: habits
    }))
}


pub fn routes() -> Vec<Route> {
    routes![register, login, get_data]
}
