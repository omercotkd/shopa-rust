use crate::models::lists::{PostItemPayload, PostShopaList, ShopaListDocument, SingleItem};
use crate::models::{DbDocument, EmbadedDocument};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::Database;
use rocket::serde::json::Json;

const LIST_COLLECTION_NAME: &str = "lists";

/// return a shopa_list document for the given id
pub async fn find_shopa_list_by_id(
    db: &Database,
    oid: ObjectId,
) -> mongodb::error::Result<Option<ShopaListDocument>> {
    let collection = db.collection::<ShopaListDocument>(LIST_COLLECTION_NAME);

    let customer_doc = collection.find_one(doc! {"_id": oid }, None).await?;

    if customer_doc.is_none() {
        return Ok(None);
    }
    let unwrapped_doc = customer_doc.unwrap();

    Ok(Some(unwrapped_doc))
}

pub async fn insert_shopa_list(
    db: &Database,
    input: Json<PostShopaList>,
) -> Result<Option<String>, mongodb::error::Error> {
    let list_collection = db.collection::<Document>(LIST_COLLECTION_NAME);

    let insert_one_result = list_collection
        .insert_one(ShopaListDocument::new_document(input.into_inner()), None)
        .await?;

    let new_list_id = match insert_one_result.inserted_id.as_object_id() {
        Some(id) => id,
        None => return Ok(None),
    };

    Ok(Some(new_list_id.to_string()))
}

pub async fn add_item_to_shopa_list(
    db: &Database,
    input: Json<PostItemPayload>,
    list_id: ObjectId,
) -> Result<Option<String>, mongodb::error::Error> {
    let new_item = SingleItem::new_document(input.into_inner());

    let list_collection = db.collection::<ShopaListDocument>(LIST_COLLECTION_NAME);

    list_collection
        .update_one(
            doc! {"_id": list_id},
            doc! {"$push": {"items": &new_item}},
            None,
        )
        .await?;

    let item_id = match new_item.get_str("_id") {
        Ok(id) => id,
        Err(_) => return Ok(None)
    };

    Ok(Some(item_id.to_string()))
}

pub async fn delete_items_from_shopa_list(
    db: &Database,
    items_ids: Vec<String>,
    list_id: ObjectId,
) -> mongodb::error::Result<String> {
    let list_collection = db.collection::<ShopaListDocument>(LIST_COLLECTION_NAME);

    list_collection
        .update_one(
            doc! {"_id": list_id},
            doc! {"$pull": {"items": {"_id": {"$in": items_ids}}}},
            None,
        )
        .await?;

    Ok("deleted".to_string())
}
