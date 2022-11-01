use crate::db::lists::{
    add_item_to_shopa_list, delete_items_from_shopa_list, find_shopa_list_by_id, insert_shopa_list,
};
use crate::helpers::{response, validators};
use crate::models::lists::{DeleteItemsPayload, PostItemPayload, PostShopaList};
use crate::models::DbDocument;
use mongodb::Database;
use rocket::serde::json::{json, Json};
use rocket::State;
use rocket::http::CookieJar;
use super::super::middleware::verify_cookie_access_token;

#[get("/?<list_id>")]
pub async fn get_list(
    db: &State<Database>,
    list_id: String,
    cookies: &CookieJar<'_>,
) -> Result<response::Success, response::Error> {

    let user_id = verify_cookie_access_token(cookies)?;
    println!("user_id: {}", user_id);
    let list_id = validators::valid_object_id(&list_id)?;

    match find_shopa_list_by_id(&db, list_id).await {
        Ok(opt) => match opt {
            Some(list) => {
                return Ok(response::ApiSuccess::build(
                    200,
                    json!({ "list": list.jsonify() }),
                ))
            }
            None => return Err(response::ApiError::build(200, "not found")),
        },
        Err(_) => return Err(response::ApiError::build(400, "not found")),
    }
}

#[post("/", format = "json", data = "<new_list>")]
pub async fn create_list(
    db: &State<Database>,
    new_list: Json<PostShopaList>,
) -> Result<response::Success, response::Error> {
    match insert_shopa_list(&db, new_list).await {
        Ok(mut id) => {
            return Ok(response::ApiSuccess::build(
                200,
                json!({"massage": "list created", "_id": id.take()}),
            ))
        }
        Err(_) => {
            return Err(response::ApiError::build(
                400,
                "error while adding the new list",
            ))
        }
    }
}

#[post("/item?<list_id>", format = "json", data = "<item>")]
pub async fn add_item_to_list(
    db: &State<Database>,
    item: Json<PostItemPayload>,
    list_id: String,
) -> Result<response::Success, response::Error> {
    let list_id = validators::valid_object_id(&list_id)?;

    let item_id = match add_item_to_shopa_list(&db, item, list_id).await {
        Ok(option) => match option {
            Some(id) => id,
            None => {
                return Err(response::ApiError::build(
                    400,
                    "error while adding new item",
                ))
            }
        },
        Err(_) => {
            return Err(response::ApiError::build(
                400,
                "error while adding new item",
            ))
        }
    };

    return Ok(response::ApiSuccess::build(
        201,
        json!({"massage": "Item added", "_id": item_id}),
    ));
}

#[delete("/item?<list_id>", format = "json", data = "<data>")]
pub async fn delete_items_from_list(
    db: &State<Database>,
    data: Json<DeleteItemsPayload>,
    list_id: String,
) -> Result<response::Success, response::Error> {
    let list_id = validators::valid_object_id(&list_id)?;

    let payload = data.into_inner();

    // i dont care for the result
    let _res = delete_items_from_shopa_list(&db, payload.items_ids, list_id).await;

    Ok(response::ApiSuccess::build(201, json!({"massage": "Items removed"})))
}
