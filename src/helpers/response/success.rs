use rocket::http::Status;
use rocket::serde::json::Value;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiSuccess {}
pub type Success = (Status, Value);

impl ApiSuccess {
    pub fn build(code: u16, data: Value) -> (Status, Value) {
        (Status { code }, data)
    }
}
