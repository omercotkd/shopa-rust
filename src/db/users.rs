use crate::models::{
    users::{NewUserPayload, UserDocument},
    DbDocument,
};
use mongodb::bson::{doc, Document};
use mongodb::Database;

const USER_COLLECTION_NAME: &str = "users";

pub async fn insert_new_user(
    db: &Database,
    new_user: NewUserPayload,
) -> mongodb::error::Result<Option<String>> {
    let user_collection = db.collection::<Document>(USER_COLLECTION_NAME);

    let insert_one_result = user_collection
        .insert_one(UserDocument::new_document(new_user), None)
        .await?;

    let new_user_id = match insert_one_result.inserted_id.as_object_id() {
        Some(id) => id,
        None => return Ok(None),
    };

    Ok(Some(new_user_id.to_string()))
}

pub async fn get_user_by_email(
    db: &Database,
    email: &str,
) -> mongodb::error::Result<Option<UserDocument>> {
    let collection = db.collection::<UserDocument>(USER_COLLECTION_NAME);

    let user = collection.find_one(doc! {"email": email }, None).await?;

    if user.is_none() {
        return Ok(None);
    }
    let user = user.unwrap();

    Ok(Some(user))
}

pub async fn get_user_by_phone(
    db: &Database,
    phone: &str,
) -> mongodb::error::Result<Option<UserDocument>> {
    let collection = db.collection::<UserDocument>(USER_COLLECTION_NAME);

    let user = collection.find_one(doc! {"phone": phone }, None).await?;

    if user.is_none() {
        return Ok(None);
    }
    let user = user.unwrap();

    Ok(Some(user))
}

pub async fn test_user_exist(db: &Database, phone: &str, email: &str) -> mongodb::error::Result<bool> {

    let collection = db.collection::<Document>(USER_COLLECTION_NAME);

    let user = collection.find_one(doc! {"$or": [{"email": email}, {"phone": phone}] }, None).await?;

    match user{
        Some(_) => return Ok(true),
        None => return Ok(false)
    }

}
