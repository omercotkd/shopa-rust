use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::models::users::UserDocument;
use jwt::{SignWithKey, VerifyWithKey};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;
use std::collections::BTreeMap;

pub fn hash_password(password: &str) -> Result<String, String> {
    let argon2: Argon2 = Argon2::default();

    let salt = SaltString::generate(&mut OsRng);

    let password_hash = match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(v) => v,
        Err(_) => return Err("error while hasing".to_string()),
    };

    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash_password: &str) -> bool {
    let argon2: Argon2 = Argon2::default();

    let parsed_hash = match PasswordHash::new(hash_password) {
        Ok(v) => v,
        Err(_) => return false,
    };

    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub fn generate_login_token(user: &UserDocument) -> Option<String> {

    let token_key: Hmac<Sha256> = Hmac::new_from_slice(env::var("SECRET_KEY").unwrap().as_bytes()).unwrap();

    let mut claims = BTreeMap::new();

    claims.insert("id", user._id.to_string());
    claims.insert("exp", "1234567890".to_string());


    match claims.sign_with_key(&token_key){
        Ok(v) => Some(v),
        Err(_) => None,
    }
}   

pub fn verify_login_token(token: &str) -> Option<String> {
    let token_key: Hmac<Sha256> = Hmac::new_from_slice(env::var("SECRET_KEY").unwrap().as_bytes()).unwrap();

    let claims: BTreeMap<String, String> = match token.verify_with_key(&token_key){
        Ok(v) => v,
        Err(_) => return None,
    };

    let exp: u32 = claims["exp"].parse().unwrap();
    
    Some(claims["id"].to_string())
}
