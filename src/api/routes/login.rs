use crate::db::users::{get_user_by_email};
use crate::helpers::{response, password::hash_password, validators};
use crate::models::users::NewUserPayload;
use mongodb::Database;
use rocket::serde::json::{json, Json};
use rocket::State;
use crate::helpers::types::{Email, PhoneNumber};


#[post("/<info>")]
pub fn login_with_email_or_phone(info: String){
    if validators::validate_email(&info){
        // preform login with email here
    }else if validators::validate_phone_number(&info){
        // preform login with phone here
    }else {
        // return invalid info response
    }
}
