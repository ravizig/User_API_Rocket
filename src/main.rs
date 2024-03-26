
mod api;
mod models;
mod repository;
pub mod helpers;

#[macro_use]
extern crate rocket;

use api::user_api::{get_all_users, get_user, get_user_using_email, hello, user_login, register_user};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![hello])
        .mount("/user", routes![register_user, get_all_users, get_user, get_user_using_email, user_login])
}