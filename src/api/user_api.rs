use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, response::status::Custom, serde::json::Json, State};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    message: String,
}

#[get("/")]
pub fn hello(db: &State<MongoRepo>) -> Result<Json<String>, Status> {
    db.hello()
}

#[post("/register", data = "<new_user>")]
pub fn user_register(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
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
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/all")]
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Status> {
    let users = db.get_all_users();
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/id/<id>")]
pub fn get_user(db: &State<MongoRepo>, id: String) -> Result<Json<User>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let user_detail = db.get_user(&id);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/email/<email>")]
pub fn get_user_using_email(db: &State<MongoRepo>, email: String) -> Result<Json<User>, Status> {
    if email.is_empty() {
        return Err(Status::BadRequest);
    };

    match db.get_user_using_email(&email) {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/login", data = "<login_data>")]
pub fn user_login(db: &State<MongoRepo>, login_data: Json<User>) -> Result<Json<String>, Json<ErrorMessage>> {
    let email = login_data.email.to_string();
    let provided_password = login_data.password.to_string();

    let login_status = db.user_login(&email, &provided_password);

    match login_status {
        Ok(user) => Ok(Json(user.to_string())),
        Err(_) => {
            let error_message = ErrorMessage {
                message: "Login failed".to_string(),
            };
            Err(Json(error_message))
        }
    }
}
