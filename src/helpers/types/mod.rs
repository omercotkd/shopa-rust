use crate::helpers::validators;
// use mongodb::bson::oid::ObjectId;
use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Deserialize)]
#[serde(try_from = "String")]
pub struct Email(String);

impl Email {
    pub fn try_new(email: String) -> Result<Self, String> {
        if validators::validate_email(&email) {
            Ok(Self(email))
        } else {
            Err(format!("Invalid email {}", email))
        }
    }
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Email {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Email::try_new(value)
    }
}

// maybe in the future use it if I need to get obi in json
// #[derive(Deserialize)]
// #[serde(try_from = "String")]
// pub struct ObjectIdFromUser(ObjectId);

// impl ObjectIdFromUser {
//     pub fn try_new(obi: String) -> Result<Self, String> {
//         if let Ok(obi) = validators::valid_object_id(&obi) {
//             Ok(Self(obi))
//         } else {
//             Err(format!("Invalid obi {}", obi))
//         }
//     }
//     pub fn into_inner(self) -> ObjectId {
//         self.0
//     }
//     pub fn inner(&self) -> &ObjectId {
//         &self.0
//     }
// }

// impl TryFrom<String> for ObjectIdFromUser {
//     type Error = String;

//     fn try_from(value: String) -> Result<Self, Self::Error> {
//         ObjectIdFromUser::try_new(value)
//     }
// }