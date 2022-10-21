use crate::db::users::{insert_new_user, test_user_exist};
use crate::helpers::{password::hash_password, response};
use crate::models::users::NewUserPayload;
use mongodb::Database;
use rocket::serde::json::{json, Json};
use rocket::State;

#[post("/", format = "json", data = "<new_user>")]
pub async fn register_new_user(
    db: &State<Database>,
    new_user: Json<NewUserPayload>,
) -> Result<response::Success, response::Error> {
    let mut new_user = new_user.into_inner();

    match test_user_exist(&db, new_user.phone.inner(), new_user.email.inner()).await {
        Ok(exist) => {
            if exist {
                return Err(response::ApiError::build(
                    412,
                    "email or phone alredy in use",
                ));
            }
        }
        Err(_) => {
            return Err(response::ApiError::build(
                400,
                "error while creating new user",
            ))
        }
    }

    let hased_password = match hash_password(&new_user.password) {
        Ok(v) => v,
        Err(e) => return Err(response::ApiError::build(400, &e)),
    };

    new_user.password = hased_password;

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
