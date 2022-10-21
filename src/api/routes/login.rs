use crate::db::users::{get_user_by_email, get_user_by_phone};
use crate::helpers::{password::verify_password, response};
use crate::models::users::{LoginWithEmailPayload, LoginWithPhonePayload};
use mongodb::Database;
use rocket::serde::json::{json, Json};
use rocket::State;

#[post("/email", format = "json", data = "<data>")]
pub async fn login_with_email(
    db: &State<Database>,
    data: Json<LoginWithEmailPayload>,
) -> Result<response::Success, response::Error> {
    let data = data.into_inner();

    let user = match get_user_by_email(db, data.email.inner()).await {
        Ok(opt) => match opt {
            Some(v) => v,
            None => return Err(response::ApiError::build(400, "")),
        },
        Err(_) => {
            return Err(response::ApiError::build(400, "error while finding user"));
        }
    };

    if !verify_password(&data.password, &user.password) {
        return Err(response::ApiError::build(400, ""));
    }

    return Ok(response::ApiSuccess::build(200, json!({})));
}

#[post("/phone", format = "json", data = "<data>")]
pub async fn login_with_phone(
    db: &State<Database>,
    data: Json<LoginWithPhonePayload>,
) -> Result<response::Success, response::Error> {
    let data = data.into_inner();

    let user = match get_user_by_phone(db, data.phone.inner()).await {
        Ok(opt) => match opt {
            Some(v) => v,
            None => return Err(response::ApiError::build(400, "")),
        },
        Err(_) => {
            return Err(response::ApiError::build(400, "error while finding user"));
        }
    };

    if !verify_password(&data.password, &user.password) {
        return Err(response::ApiError::build(400, ""));
    }

    return Ok(response::ApiSuccess::build(200, json!({"massage": "succsess"})));
}
