use crate::helpers::{response, security::{generate_login_token, verify_login_token}};
use crate::models::users::UserDocument;
use rocket::http::{Cookie, CookieJar};

pub const ACCESS_TOKEN_NAME: &str = "access_token";

pub fn set_cookie_access_token(user: &UserDocument, cookies: &CookieJar) -> Result<(), response::Error> {
    let token = match generate_login_token(&user) {
        Some(v) => v,
        None => {
            return Err(response::ApiError::build(
                400,
                "error while generating token",
            ))
        }
    };

    let cookie = Cookie::build(ACCESS_TOKEN_NAME, token)
        .http_only(true)
        .secure(true)
        .finish();

    cookies.add(cookie);

    Ok(())
}

pub fn verify_cookie_access_token(cookies: &CookieJar) -> Result<String, response::Error> {
    let token = match cookies.get(ACCESS_TOKEN_NAME) {
        Some(v) => v,
        None => {
            return Err(response::ApiError::build(
                400,
                "access token not found",
            ))
        }
    };

    let user_id = match verify_login_token(token.value()) {
        Some(v) => v,
        None => {
            return Err(response::ApiError::build(
                400,
                "access token is not valid",
            ))
        }
    };

    Ok(user_id)
}
