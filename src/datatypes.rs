use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TimeOfDay {
    pub hour: u8,
    pub minute: u8,
    pub second: u8
}

#[derive(Deserialize, Serialize)]
pub struct Interval {
    pub day: u8,
    pub month: u8,  // Since months have uneven days
    pub year: u8    // Sunce years also have uneven days
}

#[derive(Deserialize, Serialize)]
pub struct Mission {
    pub name: String,
    pub priority: i8,
    pub time: TimeOfDay,
    pub duration: Option<TimeOfDay>,
    pub repeat: Option<Interval>,
    pub is_preset: bool
}

#[derive(Deserialize, Serialize)]
pub struct Quick {
    pub name: String,
    pub description: String,
    pub reminder: Option<TimeOfDay>
}

#[derive(Deserialize, Serialize)]
pub struct Habit {
    pub name: String,
    pub streak: u16,
    pub last_completed: u64,
    pub completed_today: bool
}
