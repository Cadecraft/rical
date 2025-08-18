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

/// Verify a password
pub fn verify_password(incoming: &str, stored_hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(stored_hash).expect("Could not parse stored hash");
    Argon2::default().verify_password(incoming.as_bytes(), &parsed_hash).is_ok()
}
