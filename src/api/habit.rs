use rocket::{routes, get, post, delete};
use rocket::http::{CookieJar, Status};
use rocket::serde::json;
use crate::datatypes::{Habit, Db};
use crate::helpers;
use rocket_db_pools::Connection;

#[get("/habit?<id>")]
pub async fn get_habit(mut db: Connection<Db>, jar: &CookieJar<'_>, id: i64) -> Option<json::Json<Habit>> {
    let auth_uuid = jar.get("auth")?.value().to_string();

    let row = sqlx::query_as!(
        Habit,
        "SELECT id, name, streak, last_completed, nag_time FROM habits WHERE user_id = ? AND id = ?",
        auth_uuid,
        id
    )
    .fetch_optional(&mut **db)
    .await
    .ok()??;

    Some(json::Json(row))

}

#[post("/habit", format="json", data="<data>")]
pub async fn put_habit(mut db: Connection<Db>, jar: &CookieJar<'_>, data: json::Json<Habit>) -> Option<Status> {
    let auth_uuid = jar.get("auth")?.value().to_string();
    if !helpers::validate_uuid(auth_uuid.clone(), &mut db).await? {
        return Some(Status::Forbidden)
    }
    
    if data.id == 0 {
        sqlx::query!(
            "INSERT INTO habits (name, streak, last_completed, nag_time, user_id) VALUES (?, ?, ?, ?, ?)",
            data.name,
            data.streak,
            data.last_completed,
            data.nag_time,
            auth_uuid
        )
        .execute(&mut **db)
        .await.ok()?;
        return Some(Status::Ok)
    } else {
        sqlx::query!(
            "UPDATE habits SET name = ?, streak = ?, last_completed = ?, nag_time = ? WHERE user_id = ? AND id = ?",
            data.name,
            data.streak,
            data.last_completed,
            data.nag_time,
            auth_uuid,
            data.id
        )
        .execute(&mut **db)
        .await.ok()?;
        return Some(Status::Ok)
    }
}

#[delete("/habit?<id>")]
pub async fn delete_habit(mut db: Connection<Db>, jar: &CookieJar<'_>, id: i64) -> Option<Status> {
    let auth_uuid = jar.get("auth")?.value().to_string();
    if !helpers::validate_uuid(auth_uuid.clone(), &mut db).await? {
        return Some(Status::Forbidden)
    }

    sqlx::query!(
        "DELETE FROM habits WHERE id = ? AND user_id = ?",
        id,
        auth_uuid
    )
    .execute(&mut **db)
    .await.ok()?;

    Some(Status::NoContent)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_habit, put_habit, delete_habit]
}
