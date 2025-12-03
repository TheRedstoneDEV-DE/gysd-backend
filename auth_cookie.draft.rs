use rocket::http::{Cookie, CookieJar};

#[post("/login")]
fn login(jar: &CookieJar<'_>) -> &'static str {
    // TODO: Check for cookie before attaching it
    let token = "abc123"; // generate JWT or similar
    jar.add(Cookie::build("auth", token)
        .http_only(true)     // Important: JS cannot read it
        .secure(true)        // HTTPS required
        .same_site(SameSite::Lax) // or Strict, or None for cross-site
        .finish());
    "Logged in"
}

// ---------------------------------------------------------------------------- //

use rocket::http::CookieJar;

#[get("/profile")]
fn profile(jar: &CookieJar<'_>) -> String {
    if let Some(cookie) = jar.get("auth") {
        let token = cookie.value();
        // verify JWT or session token here
        format!("Token: {}", token)
    } else {
        "Unauthorized".into()
    }
}

