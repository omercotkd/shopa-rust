use crate::helpers::types::{Email, PhoneNumber};
use crate::models::DbDocument;
use mongodb::bson::doc;
use mongodb::bson::{oid::ObjectId, DateTime, Document};
use rocket::serde::json::{json, Value};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct NewUserPayload {
    pub first_name: String,
    pub last_name: String,
    pub email: Email,
    pub password: String,
    pub phone: PhoneNumber,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDocument {
    pub _id: ObjectId,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: DateTime,
    pub password: String,
    pub phone: String,
}

impl DbDocument for UserDocument {
    type UserInput = NewUserPayload;
    fn new_document(data: NewUserPayload) -> Document {
        doc! {
            "first_name": data.first_name,
            "last_name": data.last_name,
            "created_at": DateTime::now(),
            "email": data.email.into_inner(),
            "password": data.password,
            "phone": data.phone.into_inner()
        }
    }
    fn jsonify(self) -> Value {
        json!(
            {
                "_id": self._id.to_string(),
                "first_name": self.first_name,
                "last_name": self.last_name,
                "created_at": self.created_at.to_string(),
                "email": self.email,
                "phone": self.phone

            }
        )
    }
}
