use rocket::{get, routes};
use rocket::response::stream::{Event, EventStream};
use rocket::http::{CookieJar};
use rocket::tokio::sync::broadcast::Sender;

#[get("/update")]
pub async fn get_update(jar: &CookieJar<'_>, tx: &rocket::State<Sender<String>>) -> Option<EventStream![]> { 
    let uuid = jar.get("auth")?.value().to_string();
    let mut rx = tx.subscribe();    
    
    Some(
        EventStream!{
            let msg = rx.recv().await;
            if let Ok(message) = msg {
                if message == uuid {
                    yield Event::data("update");
                }
            }
        }
    )
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_update]
}
