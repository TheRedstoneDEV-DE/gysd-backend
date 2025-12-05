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
    pub id: i64,
    pub name: String,
    pub description: String,
    pub added_timestamp: i64,   // UNIX-Timestamp 
    pub reminder: Option<u16>   // minutes
}

#[derive(Deserialize, Serialize)]
pub struct Habit {
    pub id: i64,
    pub name: String,
    pub tracked_sites: Vec<String>, // List of tracked websites
    pub time_limit: i64,            // minutes
    pub streak: i64,                // days
    pub last_completed: i64,        // UNIX-Timestamp
    pub completed_today: bool,
    pub nag_interval: i64           // minutes 
}
