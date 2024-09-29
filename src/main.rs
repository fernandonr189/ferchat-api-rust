mod controllers;
mod models;
mod util;
use controllers::auth::{hello_token, login, signup};
use controllers::catchers::{internal_server_error, not_found, unauthorized};
use dotenvy::dotenv;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![login])
        .mount("/", routes![hello_token])
        .mount("/", routes![signup])
        .register("/", catchers![not_found])
        .register("/", catchers![unauthorized])
        .register("/", catchers![internal_server_error])
}
