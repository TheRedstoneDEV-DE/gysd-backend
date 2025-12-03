use crate::datatypes::Mission; 

#[get("/get_missions")]
pub fn get_missions() -> Option<Vec<Mission>> {
    // TODO: Database import, cookie auth
    Some()
}

pub fn routes() -> Vec<rocket::Route> {
    routes![];
}
