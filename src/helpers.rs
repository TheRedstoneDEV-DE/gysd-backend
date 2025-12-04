use crate::datatypes::{Login, Db, Mission};
use rocket::serde::json;
use rocket_db_pools::Connection;
use argon2::{Argon2, PasswordVerifier};
use argon2::password_hash::{PasswordHash};
use sqlx::Acquire;

pub async fn validate_auth(login: json::Json<Login>, mut db: Connection<Db>) -> Option<(bool,String)> {
    let row = sqlx::query!(
        "SELECT password_hash, uuid FROM users WHERE name = ?",
        login.username
    )
    .fetch_optional(&mut **db)
    .await
    .ok()??;
    
    let hash = PasswordHash::new(&row.password_hash).ok()?;
    
    Some((Argon2::default().verify_password(login.password.as_bytes(), &hash)
    .is_ok(), row.uuid?))
}

pub async fn validate_uuid(uuid: String, db: &mut Connection<Db>) -> Option<bool> {
    let conn = db.acquire().await.ok()?;
    Some(
        sqlx::query("SELECT 1 FROM users WHERE uuid = ? LIMIT 1")
            .bind(&uuid)
            .fetch_optional(conn)
            .await.ok()?
            .is_some()
    )
}

pub async fn get_missions(uuid: String, mut db: Connection<Db>) -> Option<Vec<Mission>> {
    let rows = sqlx::query_as!(
        Mission,
        "SELECT id, name, priority, time, duration, repeat, is_preset FROM missions WHERE user_id = ?",
        uuid
    )
    .fetch_all(&mut **db)
    .await
    .ok()?;

    Some(rows)
}
