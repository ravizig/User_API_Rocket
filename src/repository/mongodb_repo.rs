extern crate dotenv;
use std::env;

use bcrypt::{hash, verify, DEFAULT_COST};
use dotenv::dotenv;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::InsertOneResult,
    sync::{Client, Collection},
};
use rocket::{http::Status, serde::json::Json};
use serde::de::Error as _;

use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error in loading env variables"),
        };

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("RocketDB");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub fn hello(&self) -> Result<Json<String>, Status> {
        Ok(Json(String::from("Hello Form Rocket with MongoDB in Rust")))
    }

    pub fn register_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let filter = doc! {"email": new_user.email.clone()};

        let role = if new_user.role == true {
            new_user.role
        } else {
            false
        };

        let hashed_password =
            hash(new_user.password, DEFAULT_COST).expect("Error in hashing password");

        let new_doc = User {
            id: None,
            username: new_user.username,
            email: new_user.email,
            password: hashed_password,
            role,
        };

        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");

        if !user_detail.is_some() {
            let user = self
                .col
                .insert_one(new_doc, None)
                .ok()
                .expect("Error in creating user");

            return Ok(user);
        } else {
            return Err(Error::custom("User already exists"));
        }
    }

    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of users");

        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }

    pub fn get_user_using_email(&self, email: &String) -> Result<User, Error> {
        let filter = doc! {"email": email};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub fn user_login(
        &self,
        email: &String,
        provided_password: &String,
    ) -> Result<Json<String>, Json<String>> {
        let user_result = self.get_user_using_email(email);

        let user = match user_result {
            Ok(user) => user,
            _ => return Err(Json("User does not exist".to_string())),
        };

        let stored_password = user.password.clone();

        match verify(provided_password, &stored_password) {
            Ok(valid) => {
                if valid {
                    return Ok(Json("Login Successful".to_string()));
                } else {
                    return Err(Json("Invalid Password".to_string()));
                }
            }
            Err(_) => return Err(Json("Error in verifying password".to_string())),
        };
    }
}
