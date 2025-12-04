use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("sqlite_db")]
pub struct Db(sqlx::SqlitePool);


#[derive(Deserialize, Serialize)]
pub struct Login {
    pub username: String,
    pub password: String
}

#[derive(Deserialize, Serialize)]
pub struct UserData {
    pub missions: Option<Vec<Mission>>,
    pub quicks: Option<Vec<Quick>>,
    pub habits: Option<Vec<Habit>>
}

#[derive(Deserialize, Serialize)]
pub struct Interval {
    pub day: u8,
    pub month: u8,              // Since months have uneven days
    pub year: u8                // Sunce years also have uneven days
}

#[derive(Deserialize, Serialize)]
pub struct Mission {
    pub id: i64,
    pub name: String,
    pub priority: i64,          // low 0 - 10 high
    pub time: i64,              // UNIX time
    pub duration: Option<i64>,  // minutes
    pub repeat: Option<i64>,    // positive - days | negative - months
    pub is_preset: bool
}

#[derive(Deserialize, Serialize)]
pub struct Quick {
    pub name: String,
    pub description: String,
    pub reminder: Option<u16>   // minutes
}

#[derive(Deserialize, Serialize)]
pub struct Habit {
    pub name: String,
    pub streak: u16,
    pub last_completed: u64,
    pub completed_today: bool
}
