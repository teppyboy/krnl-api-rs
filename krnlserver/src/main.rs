#[macro_use] extern crate rocket;

#[get("/version")]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to krnlserver!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![version])
}
