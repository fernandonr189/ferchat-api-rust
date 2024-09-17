use rocket::serde::{json::Json, Serialize};

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    description: &'r str,
    complete: bool,
}

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

#[get("/json/<name>")]
fn json_hello(name: &str) -> Json<Task> {
    let task = Task {
        description: name,
        complete: true,
    };
    Json(task)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![world])
        .mount("/", routes![hello])
        .mount("/hello", routes![json_hello])
}
