mod controllers;
mod models;
mod util;
use controllers::auth::hello_token;
use controllers::auth::login;
use controllers::auth::signup;
use controllers::catchers::not_found;
use controllers::catchers::unauthorized;
use controllers::users::get_users;
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
        .mount("/users", routes![get_users])
        .register("/", catchers![not_found])
        .register("/", catchers![unauthorized])
}
