
mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::user_api::{get_all_users, get_user, get_user_using_email, hello, user_login, user_register};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![hello])
        .mount("/user", routes![user_register, get_all_users, get_user, get_user_using_email, user_login])
}