
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

fn default_role() -> bool {
    false
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: Option<String>,
    pub email: String,
    pub password: String,
    #[serde(default = "default_role")]
    pub role: bool,
}
