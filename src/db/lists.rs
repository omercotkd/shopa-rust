use crate::models::lists::{
    NewShopaList, PostItemPayload, PostShopaList, ShopaListDocument, SingleItem,
};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use rocket::serde::json::Json;

const LIST_COLLECTION_NAME: &str = "lists";

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
) -> mongodb::error::Result<String> {
    let list_collection = db.collection::<NewShopaList>(LIST_COLLECTION_NAME);

    let insert_one_result = list_collection
        .insert_one(ShopaListDocument::new(input.into_inner()), None)
        .await?;

    Ok(insert_one_result.inserted_id.to_string())
}

pub async fn add_item_to_shopa_list(
    db: &Database,
    input: Json<PostItemPayload>,
    list_id: ObjectId,
) -> mongodb::error::Result<String> {
    let new_item = SingleItem::new(input.into_inner());

    let list_collection = db.collection::<ShopaListDocument>(LIST_COLLECTION_NAME);

    list_collection
        .update_one(
            doc! {"_id": list_id},
            doc! {"$push": {"items": new_item.to_document()}},
            None,
        )
        .await?;

    Ok(new_item._id)
}

pub async fn delete_items_from_shopa_list(db: &Database, items_ids: Vec<String>, list_id: ObjectId) -> mongodb::error::Result<String> {

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
