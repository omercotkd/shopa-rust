pub mod lists;
pub mod users;
use mongodb::bson;
use rocket::serde::json;

pub trait DbDocument {
    type UserInput;
    fn new_document(_: Self::UserInput) -> bson::Document;
    fn jsonify(self) -> json::Value;
}

pub trait EmbadedDocument{
    type UserInput;
    fn new_document(_: Self::UserInput) -> bson::Document;
}
