use rocket::{routes, get, post, delete};
use rocket::serde::json;
use rocket::http::{CookieJar, Status};
use crate::datatypes::{Quick, Db};
use crate::helpers;
use rocket_db_pools::Connection;

#[get("/quick?<id>")]
pub async fn get_quick(mut db: Connection<Db>, jar: &CookieJar<'_>, id: i64) -> Option<json::Json<Quick>> {
    let auth_uuid = jar.get("auth")?.value().to_string();
    
    let row = sqlx::query_as!(
        Quick,
        "SELECT id, name, added_timestamp, reminder FROM quicks WHERE user_id = ? AND id = ?",
        auth_uuid,
        id
    )
    .fetch_optional(&mut **db)
    .await
    .ok()??;

    Some(json::Json(row))
}

#[post("/quick", format="json", data="<data>")]
pub async fn put_quick(mut db: Connection<Db>, jar: &CookieJar<'_>, data: json::Json<Quick>) -> Option<Status> {
    let auth_uuid = jar.get("auth")?.value().to_string();
    if !helpers::validate_uuid(auth_uuid.clone(), &mut db).await? {
        return Some(Status::Forbidden)
    }
    
    if data.id == 0 {
        sqlx::query!(
            "INSERT INTO quicks (name, added_timestamp, reminder, user_id) VALUES (?, ?, ?, ?)",
            data.name,
            data.added_timestamp,
            data.reminder,
            auth_uuid
        )
        .execute(&mut **db)
        .await.ok()?;
        return Some(Status::Ok)
    } else {
        sqlx::query!(
            "UPDATE quicks SET name = ?, added_timestamp = ?, reminder = ? WHERE user_id = ? AND id = ?",
            data.name,
            data.added_timestamp,
            data.reminder,
            auth_uuid,
            data.id
        )
        .execute(&mut **db)
        .await.ok()?;
        return Some(Status::Ok)
    }
}

#[delete("/quick?<id>")]
pub async fn delete_quick(mut db: Connection<Db>, jar: &CookieJar<'_>, id: i64) -> Option<Status> {
    let auth_uuid = jar.get("auth")?.value().to_string();
    if !helpers::validate_uuid(auth_uuid.clone(), &mut db).await? {
        return Some(Status::Forbidden)
    }

    sqlx::query!(
        "DELETE FROM quicks WHERE id = ? AND user_id = ?",
        id,
        auth_uuid
    )
    .execute(&mut **db)
    .await.ok()?;

    Some(Status::NoContent)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_quick, put_quick, delete_quick]
}
