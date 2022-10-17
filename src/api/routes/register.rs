use crate::db::users::{get_user_by_email, insert_new_user};
use crate::helpers::response;
use crate::models::users::NewUserPayload;
use mongodb::Database;
use rocket::serde::json::{json, Json};
use rocket::State;

#[post("/", format = "json", data = "<new_user>")]
pub async fn register_new_user(
    db: &State<Database>,
    new_user: Json<NewUserPayload>,
) -> Result<response::Success, response::Error> {
    let new_user = new_user.into_inner();

    match get_user_by_email(&db, new_user.email.inner()).await {
        Ok(opt) => match opt {
            Some(_) => return Err(response::ApiError::build(412, "email alredy in use")),
            None => {}
        },
        Err(_) => {
            return Err(response::ApiError::build(
                400,
                "error while creating new user",
            ))
        }
    }

    let user_id = match insert_new_user(&db, new_user).await {
        Ok(opt) => match opt {
            Some(id) => id,
            None => {
                return Err(response::ApiError::build(
                    400,
                    "error while creating new user",
                ))
            }
        },
        Err(_) => {
            return Err(response::ApiError::build(
                400,
                "error while creating new user",
            ))
        }
    };

    return Ok(response::ApiSuccess::build(
        200,
        json!({"massage": "user added", "_id": user_id}),
    ));
}
