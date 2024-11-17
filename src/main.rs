mod controllers;
mod models;
mod util;
use controllers::auth::{hello_token, login, signup};
use controllers::catchers::{internal_server_error, not_found, unauthorized};
use controllers::chat::msg;
use controllers::friends::{accept_request, cancel_request, list_friends, send_request};
use controllers::options::get_options;
use controllers::session::session;
use dotenvy::dotenv;
use models::event_server::EventServer;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let event_server = EventServer::default();
    dotenv().ok();
    rocket::build()
        .configure(rocket::Config {
            address: "0.0.0.0".parse().unwrap(),
            port: 8000,
            ..Default::default()
        })
        .manage(event_server)
        .mount("/", routes![login, hello_token, signup, session, msg, get_options])
        .mount(
            "/friends",
            routes![send_request, accept_request, list_friends, cancel_request],
        )
        .register(
            "/",
            catchers![not_found, unauthorized, internal_server_error],
        )
}
