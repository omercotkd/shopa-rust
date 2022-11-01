use super::super::middleware::set_cookie_access_token;
use crate::db::users::{get_user_by_email, get_user_by_phone};
use crate::helpers::{response, security::verify_password};
use crate::models::users::{LoginWithEmailPayload, LoginWithPhonePayload};
use mongodb::Database;
use rocket::http::CookieJar;
use rocket::serde::json::{json, Json};
use rocket::State;

#[post("/email", format = "json", data = "<data>")]
pub async fn login_with_email(
    db: &State<Database>,
    data: Json<LoginWithEmailPayload>,
    cookies: &CookieJar<'_>,
) -> Result<response::Success, response::Error> {
    let data = data.into_inner();

    let user = match get_user_by_email(db, data.email.inner()).await {
        Ok(opt) => match opt {
            Some(v) => v,
            None => {
                return Err(response::ApiError::build(
                    400,
                    "email or password is incorrect",
                ))
            }
        },
        Err(_) => {
            return Err(response::ApiError::build(400, "error while finding user"));
        }
    };

    if !verify_password(&data.password, &user.password) {
        return Err(response::ApiError::build(
            400,
            "email or password is incorrect",
        ));
    };

    set_cookie_access_token(&user, cookies)?;

    return Ok(response::ApiSuccess::build(
        200,
        json!({"massage": "succsess"}),
    ));
}

#[post("/phone", format = "json", data = "<data>")]
pub async fn login_with_phone(
    db: &State<Database>,
    data: Json<LoginWithPhonePayload>,
    cookies: &CookieJar<'_>,
) -> Result<response::Success, response::Error> {
    let data = data.into_inner();

    let user = match get_user_by_phone(db, data.phone.inner()).await {
        Ok(opt) => match opt {
            Some(v) => v,
            None => {
                return Err(response::ApiError::build(
                    400,
                    "phone or password is incorrect",
                ))
            }
        },
        Err(_) => {
            return Err(response::ApiError::build(400, "error while finding user"));
        }
    };

    if !verify_password(&data.password, &user.password) {
        return Err(response::ApiError::build(
            400,
            "phone or password is incorrect",
        ));
    }

    set_cookie_access_token(&user, cookies)?;

    return Ok(response::ApiSuccess::build(
        200,
        json!({"massage": "succsess"}),
    ));
}
