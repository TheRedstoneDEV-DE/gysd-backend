use rocket::{self, launch};
use rocket_db_pools::Database;
use rocket::fairing::AdHoc;

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
        "#
    )
    .execute(pool)
    .await
    .expect("migration failed");
}


#[launch]
async fn rocket() -> _ {
    // TODO: mount all other API submodules
    // TODO: add fileserver for static content
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::on_ignite("Run Migrations", |rocket| async {
            let db_pool = Db::fetch(&rocket).unwrap();
            run_migrations(db_pool).await;
            rocket
        }))
        .mount("/api/user/", api::user::routes())
        .mount("/api/mission/", api::mission::routes())
}
