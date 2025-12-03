use rocket::{self, launch, routes};
mod datatypes;

#[launch]
fn rocket() -> _ {
    // TODO: mount all other API submodules
    // TODO: add fileserver for static content
    rocket::build().mount("/api", routes![])
}
