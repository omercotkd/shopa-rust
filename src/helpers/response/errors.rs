use rocket::http::Status;
use rocket::serde::json::{json, Value};


pub struct ApiError {}
pub type Error = (Status, Value);

impl ApiError {
    pub fn build(code: u16, description: &str) -> Error {
        (Status { code }, json!({ "error": description }))
    }
    pub fn build_with_id(code: u16, description: &str, id: &str) -> Error {
        (Status { code }, json!({ "error": description, "id": id }))
    }
}
