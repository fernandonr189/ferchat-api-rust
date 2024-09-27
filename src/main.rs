mod controllers;
mod models;
mod util;
use controllers::users::get_users;
use controllers::catchers::not_found;
use controllers::auth::login;
use controllers::auth::hello_token;
use dotenvy::dotenv;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![login])
        .mount("/", routes![hello_token])
        .mount("/users", routes![get_users])
        .register("/", catchers![not_found])
}
