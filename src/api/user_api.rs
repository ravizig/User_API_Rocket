use crate::{
    helpers::message::message_fn, models::user_model::User, repository::mongodb_repo::MongoRepo,
};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    pub message: String,
}

#[get("/")]
pub fn hello(db: &State<MongoRepo>) -> Result<Json<String>, Status> {
    db.hello()
}

#[post("/register", data = "<new_user>")]
pub fn register_user(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Json<ErrorMessage>> {
    let data = User {
        id: None,
        username: new_user.username.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
        role: if Some(new_user.role).is_some() {
            new_user.role.to_owned()
        } else {
            false
        },
    };
    let user_detail = db.register_user(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(message_fn(e.to_string())),
    }
}

#[get("/all")]
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Json<ErrorMessage>> {
    let users = db.get_all_users();
    match users {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err(message_fn(e.to_string())),
    }
}

#[get("/id/<id>")]
pub fn get_user(db: &State<MongoRepo>, id: String) -> Result<Json<User>, Json<ErrorMessage>> {
    if id.is_empty() {
        return Err(message_fn("Email cannot be empty".to_string()));
    };
    let user_detail = db.get_user(&id);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(message_fn(e.to_string())),
    }
}

#[get("/email/<email>")]
pub fn get_user_using_email(
    db: &State<MongoRepo>,
    email: String,
) -> Result<Json<User>, Json<ErrorMessage>> {
    if email.is_empty() {
        return Err(message_fn("Email cannot be empty".to_string()));
    };

    match db.get_user_using_email(&email) {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(message_fn(e.to_string())),
    }
}

#[post("/login", data = "<login_data>")]
pub fn user_login(
    db: &State<MongoRepo>,
    login_data: Json<User>,
) -> Result<Json<String>, Json<ErrorMessage>> {
    let email = login_data.email.to_string();
    let provided_password = login_data.password.to_string();

    let login_details = db.user_login(&email, &provided_password);

    match login_details {
        Ok(user) => Ok(Json(user.to_string())),
        Err(e) => Err(message_fn(e.to_string())),
    }
}
