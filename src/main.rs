use rocket::{self, launch};
use rocket_db_pools::Database;
use rocket::fairing::AdHoc;
use rocket::fs::FileServer;

use crate::datatypes::Db;

mod api;
mod datatypes;
mod helpers;

async fn run_migrations(pool: &sqlx::SqlitePool) {
    // create standard DB - Notes:
    // duration - minutes
    // repeat - days / negative - months
    // time - unix timestamp
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            uuid TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            password_hash TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS missions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id TEXT NOT NULL,
            name TEXT NOT NULL,
            time INTEGER NOT NULL,
            priority INTEGER NOT NULL,
            is_preset BOOL NOT NULL DEFAULT false,
            duration INTEGER,
            repeat INTEGER,
            FOREIGN KEY (user_id)
                REFERENCES users(uuid)
                ON DELETE CASCADE
                ON UPDATE CASCADE
        );
        CREATE TABLE IF NOT EXISTS quicks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id TEXT NOT NULL,
            name TEXT NOT NULL,
            added_timestamp INTEGER NOT NULL,
            reminder INTEGER,
            FOREIGN KEY (user_id)
                REFERENCES users(uuid)
                ON DELETE CASCADE
                ON UPDATE CASCADE
        );
        CREATE TABLE IF NOT EXISTS habits (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id TEXT NOT NULL,
            name TEXT NOT NULL,
            streak INTEGER NOT NULL DEFAULT 0,
            last_completed INTEGER NOT NULL DEFAULT 0,
            nag_time INTEGER,
            FOREIGN KEY (user_id)
                REFERENCES users(uuid)
                ON DELETE CASCADE
                ON UPDATE CASCADE
        );
        "#
    )
    .execute(pool)
    .await
    .expect("migration failed");
}


#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::on_ignite("Run Migrations", |rocket| async {
            let db_pool = Db::fetch(&rocket).unwrap();
            run_migrations(db_pool).await;
            rocket
        }))
        .mount("/api/user/", api::user::routes())
        .mount("/api/", api::mission::routes())
        .mount("/api/", api::quick::routes())
        .mount("/api/", api::habit::routes())
        .mount("/", FileServer::from("static"))
}
