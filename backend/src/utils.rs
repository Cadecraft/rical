use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString
    },
    Argon2
};

use hmac::{Hmac, Mac};
use jwt::{ SignWithKey, VerifyWithKey };
use sha2::Sha256;
use std::collections::BTreeMap;

use crate::config;

/// Hash a password
pub fn hash_password(password: &str) -> String {
    // See docs: https://docs.rs/argon2/latest/argon2/
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Could not hash password").to_string();
    password_hash
}

/// Return whether the incoming password matches the stored one
pub fn verify_password(incoming: &str, stored_hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(stored_hash).expect("Could not parse stored hash");
    Argon2::default().verify_password(incoming.as_bytes(), &parsed_hash).is_ok()
}

fn create_hmac_key() -> Hmac<Sha256> {
    let jwt_secret = &config::get_config()["JWT_SECRET"];
    Hmac::new_from_slice(jwt_secret.as_bytes())
        .expect("Could not generate key")
}

/// Create and sign a JWT for auth with the user ID
pub fn create_jwt(user_id: i64) -> String {
    // See docs: https://docs.rs/jwt/latest/jwt/
    // TODO: expiration?
    let key = create_hmac_key();
    let mut claims = BTreeMap::new();
    claims.insert("sub", user_id);
    let token_str = claims.sign_with_key(&key).expect("Could not sign");
    token_str
}

/// Verify a JWT and return the sub claim with the ID
pub fn verify_jwt(incoming_token: &str) -> Option<i64> {
    let key = create_hmac_key();

    let verif_res: Result<BTreeMap<String, i64>, jwt::Error> = incoming_token.verify_with_key(&key);
    match verif_res {
        Ok(claims) => match claims.get("sub") {
            Some(res) => Some(res.clone()),
            None => None
        },
        Err(_) => None
    }
}
