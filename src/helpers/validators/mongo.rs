use mongodb::bson::oid::ObjectId;
use crate::helpers::response;

pub fn valid_object_id(obi_str: &str) -> Result<ObjectId, response::Error> {
    match ObjectId::parse_str(obi_str) {
        Ok(obi) => Ok(obi),
        Err(_) => {
            return Err(response::ApiError::build_with_id(400, "invalid Id", obi_str));
        }
    }
}
