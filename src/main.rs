mod auth;
mod client;
mod routes;

use dotenv::dotenv;
use routes::routes;

#[macro_use]
extern crate rocket;
extern crate base64;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes())
}
