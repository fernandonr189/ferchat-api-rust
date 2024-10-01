mod controllers;
mod models;
mod util;
use controllers::auth::{hello_token, login, signup};
use controllers::catchers::{internal_server_error, not_found, unauthorized};
use controllers::chat::echo_stream;
use controllers::friends::{accept_request, cancel_request, list_friends, send_request};
use dotenvy::dotenv;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![login, hello_token, signup, echo_stream])
        .mount(
            "/friends",
            routes![send_request, accept_request, list_friends, cancel_request],
        )
        .register(
            "/",
            catchers![not_found, unauthorized, internal_server_error],
        )
}
