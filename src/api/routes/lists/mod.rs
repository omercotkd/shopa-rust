use crate::db::lists::{
    add_item_to_shopa_list, delete_items_from_shopa_list, find_shopa_list_by_id, insert_shopa_list,
};
use crate::models::lists::{DeleteItemsPayload, PostItemPayload, PostShopaList};
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::State;

#[get("/?<list_id>")]
pub async fn get_list(db: &State<Database>, list_id: String) -> (Status, Value) {
    let list_id = match ObjectId::parse_str(&list_id) {
        Ok(v) => v,
        Err(_) => {
            return (Status { code: 400 }, json!({"error": "invalid id"}));
        }
    };

    match find_shopa_list_by_id(&db, list_id).await {
        Ok(res) => match res {
            Some(list) => (Status { code: 200 }, json!({ "list": list.jsonify() })),
            None => (Status { code: 200 }, json!({"massage": "not found"})),
        },
        Err(_) => (Status { code: 400 }, json!({"list": "list"})),
    }
}

#[post("/", format = "json", data = "<new_list>")]
pub async fn create_list(db: &State<Database>, new_list: Json<PostShopaList>) -> (Status, Value) {
    match insert_shopa_list(&db, new_list).await {
        Ok(id) => (
            Status { code: 200 },
            json!({"massage": "list created", "_id": id}),
        ),
        Err(_) => {
            return (
                Status { code: 400 },
                json!({"error": "error while adding the new list"}),
            )
        }
    }
}

#[post("/item?<list_id>", format = "json", data = "<item>")]
pub async fn add_item_to_list(
    db: &State<Database>,
    item: Json<PostItemPayload>,
    list_id: String,
) -> (Status, Value) {
    let list_id = match ObjectId::parse_str(&list_id) {
        Ok(v) => v,
        Err(_) => {
            return (Status { code: 400 }, json!({"error": "invalid id"}));
        }
    };

    let item_id = match add_item_to_shopa_list(&db, item, list_id).await {
        Ok(v) => v,
        Err(_) => {
            return (
                Status { code: 400 },
                json!({"error": "error while adding new item"}),
            )
        }
    };

    (
        Status { code: 201 },
        json!({"massage": "Item added", "_id": item_id}),
    )
}

#[delete("/item?<list_id>", format = "json", data = "<data>")]
pub async fn delete_items_from_list(
    db: &State<Database>,
    data: Json<DeleteItemsPayload>,
    list_id: String,
) -> (Status, Value) {
    let list_id = match ObjectId::parse_str(&list_id) {
        Ok(v) => v,
        Err(_) => {
            return (Status { code: 400 }, json!({"error": "invalid id"}));
        }
    };
    let payload = data.into_inner();

    // i dont care for the result
    let _res = delete_items_from_shopa_list(&db, payload.items_ids, list_id).await;

    (Status { code: 200 }, json!({"massage": "Items removed"}))
}
