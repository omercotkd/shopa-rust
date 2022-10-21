use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

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
