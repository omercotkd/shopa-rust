use mongodb::bson::doc;
use mongodb::bson::{oid::ObjectId, DateTime, Document};
use rocket::serde::json::{json, Value};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::{DbDocument, EmbadedDocument};

#[derive(Deserialize, Debug)]
pub struct PostItemPayload {
    pub name: String,
    pub amount: i32,
    pub unit: String,
    pub priority: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteItemsPayload {
    pub items_ids: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct PostShopaList {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleItem {
    pub _id: String,
    pub name: String,
    pub amount: i32,
    pub unit: String,
    pub priority: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShopaListDocument {
    pub _id: ObjectId,
    pub name: String,
    pub created_at: DateTime,
    pub items: Vec<SingleItem>,
}

impl DbDocument for ShopaListDocument {
    type UserInput = PostShopaList;
    fn new_document(data: PostShopaList) -> Document {
        doc! {
            "name": data.name,
            "created_at": DateTime::now(),
            "items": [],
        }
    }
    fn jsonify(self) -> Value {
        json!(
            {
                "_id": self._id.to_string(),
                "name": self.name,
                "created_at": self.created_at.to_string(),
                "items": self.items,
            }
        )
    }
}

impl EmbadedDocument for SingleItem {
    type UserInput = PostItemPayload;
    fn new_document(data: Self::UserInput) -> Document {
        doc! {
            "_id": Uuid::new_v4().to_string(),
            "name": data.name,
            "amount": data.amount,
            "unit": data.unit,
            "priority": data.priority,
        }
    }
}
