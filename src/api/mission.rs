use rocket::{routes, get, post, delete, State};
use rocket::serde::json;
use rocket::http::{CookieJar, Status};
use rocket::tokio::sync::broadcast::Sender;
use crate::datatypes::{Mission, Db};
use crate::helpers;
use rocket_db_pools::Connection;

#[get("/mission?<id>")]
pub async fn get_mission(mut db: Connection<Db>, jar: &CookieJar<'_>, id: i64) -> Option<json::Json<Mission>> {
    let auth_uuid = jar.get("auth")?.value().to_string();

    let row = sqlx::query_as!(
Mission,
        "SELECT id, name, priority, time, duration, repeat, is_preset FROM missions WHERE user_id = ? AND id = ?",
        auth_uuid,
        id
    )
    .fetch_optional(&mut **db)
    .await
    .ok()??;

    Some(json::Json(row))
}

#[post("/mission", format="json", data="<data>")]
pub async fn put_mission(mut db: Connection<Db>, jar: &CookieJar<'_>, data: json::Json<Mission>, tx: &State<Sender<String>>) -> Option<Status> {
    let auth_uuid = jar.get("auth")?.value().to_string();
    if !helpers::validate_uuid(auth_uuid.clone(), &mut db).await? {
        return Some(Status::Forbidden)
    }
    if data.id == 0 {
        sqlx::query!(
            "INSERT INTO missions (name, priority, time, duration, repeat, is_preset, user_id) VALUES (?, ?, ?, ?, ?, ?, ?)",
            data.name,
            data.priority,
            data.time,
            data.duration,
            data.repeat,
            data.is_preset,
            auth_uuid
        )
        .execute(&mut **db)
        .await.ok()?;
    } else {
        sqlx::query!(
                "UPDATE missions SET name = ?, priority = ?, time = ?, duration = ?, repeat = ?, is_preset = ? WHERE user_id = ? AND id = ?",
                data.name,
                data.priority,
                data.time,
                data.duration,
                data.repeat,
                data.is_preset,
                auth_uuid,
                data.id
            )
            .execute(&mut **db)
            .await.ok()?;
    }

    // send update event to other clients
    let _ = tx.send(auth_uuid);

    Some(Status::Ok)
}

#[delete("/mission?<id>")]
pub async fn delete_mission(mut db: Connection<Db>, jar: &CookieJar<'_>, id: i64, tx: &State<Sender<String>>) -> Option<Status> {
    let auth_uuid = jar.get("auth")?.value().to_string();
    if !helpers::validate_uuid(auth_uuid.clone(), &mut db).await? {
        return Some(Status::Forbidden)
    }
    sqlx::query!(
            "DELETE FROM missions WHERE id = ? AND user_id = ?",
            id,
            auth_uuid
    )
    .execute(&mut **db)
    .await.ok()?;

    // send update event to other clients
    let _ = tx.send(auth_uuid);

    Some(Status::NoContent)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_mission, put_mission, delete_mission]
}
