use mongodb::bson::doc;
use mongodb::bson::{oid::ObjectId, DateTime, Document};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct PostItemPayload {
    pub name: String,
    pub amount: u8,
    pub unit: String,
    pub priority: u8,
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

#[derive(Serialize, Deserialize)]
pub struct NewShopaList {
    name: String,
    created_at: DateTime,
    items: Vec<SingleItem>,
}

#[derive(Serialize, Deserialize)]
pub struct JsonShopaList {
    pub _id: String,
    pub name: String,
    pub created_at: String,
    pub items: Vec<SingleItem>,
}

impl ShopaListDocument {
    pub fn new(input: PostShopaList) -> NewShopaList {
        NewShopaList {
            name: input.name,
            created_at: DateTime::now(),
            items: vec![],
        }
    }
    pub fn jsonify(self) -> JsonShopaList {
        JsonShopaList {
            _id: self._id.to_string(),
            name: self.name,
            created_at: self.created_at.to_string(),
            items: self.items,
        }
    }

    pub fn _clone_to_jsonify(&self) -> JsonShopaList {
        JsonShopaList {
            _id: self._id.to_string(),
            name: self.name.to_string(),
            created_at: self.created_at.to_string(),
            items: self.items.clone(),
        }
    }
}

impl SingleItem {
    pub fn new(input: PostItemPayload) -> SingleItem {
        SingleItem {
            _id: Uuid::new_v4().to_string(),
            name: input.name,
            amount: input.amount as i32,
            unit: input.unit,
            priority: input.priority as i32,
        }
    }
    pub fn to_document(&self) -> Document {
        doc! {"_id": &self._id, "name": &self.name, "amount": self.amount, "unit": &self.unit, "priority": self.priority}
    }
}
