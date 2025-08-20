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
use jwt::{ SignWithKey, VerifyWithKey, Error };
use sha2::Sha256;
use std::collections::BTreeMap;
use axum::http::header::HeaderMap;
use axum_extra::{headers::{Authorization, authorization::Bearer}, TypedHeader};

use std::env;

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
    // TODO: load configuration in main to make sure fully configred
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Hmac::new_from_slice(jwt_secret.as_bytes())
        .expect("Could not generate key")
}

/// Create and sign a JWT for auth with the user ID
pub fn create_jwt(user_id: i64) -> String {
    // See docs: https://docs.rs/jwt/latest/jwt/
    // TODO: expiration?
    let key = create_hmac_key();
    let mut claims = BTreeMap::new();
    //let user_id_str = user_id.to_string();
    claims.insert("sub", user_id);
    let token_str = claims.sign_with_key(&key).expect("Could not sign");
    token_str
}

/// Verify a JWT and return the sub claim with the ID
pub fn verify_jwt(incoming_token: &str) -> Option<i64> {
    let key = create_hmac_key();

    println!("Verifying");
    let verif_res: Result<BTreeMap<String, i64>, jwt::Error> = incoming_token.verify_with_key(&key);
    match verif_res {
        Ok(claims) => {
            println!("Claims ok");
            match claims.get("sub") {
                Some(res) => {
                    println!("Sub some");
                    Some(res.clone())
                },
                None => None
            }
        }
        Err(err) => {
            println!("With error: {}", err);
            None
        }
    }
}
