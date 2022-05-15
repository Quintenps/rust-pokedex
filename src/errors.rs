use serde_json::{json, Value};


#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[catch(422)]
pub fn unprocessable_entity() -> Value {
    json!({
        "status": "error",
        "reason": "Couldn't process entity"
    })
}

#[catch(500)]
pub fn internal_err() -> Value {
    json!({
        "status": "error",
        "reason": "Internal server error"
    })
}

#[catch(400)]
pub fn bad_request() -> Value {
    json!({
        "status": "error",
        "reason": "The request could not be understood by the server due to malformed syntax."
    })
}