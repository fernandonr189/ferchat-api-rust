#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "This is the index!"
}

#[get("/world")]
fn world() -> &'static str {
    "You called the /world endpoint!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![world])
        .mount("/", routes![hello])
}
