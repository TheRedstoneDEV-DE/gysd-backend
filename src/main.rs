use rocket::{self, launch, routes};
use rocket_db_pools::Database;
use rocket::fairing::AdHoc;

use crate::datatypes::Db;

mod api;
mod datatypes;

async fn run_migrations(pool: &sqlx::SqlitePool) {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            uuid TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            password_hash TEXT NOT NULL
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
        .mount("/api", api::login::routes())
}
