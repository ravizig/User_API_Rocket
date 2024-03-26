use rocket::serde::json::Json;

use crate::api::user_api::ErrorMessage;


pub fn message_fn(message: String) -> Json<ErrorMessage> {
    let message = ErrorMessage { message };

    return Json(message);
}


