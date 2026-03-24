use argon2::{
    password_hash::{SaltString, PasswordHash, PasswordHasher, PasswordVerifier},
    Argon2,
};
use argon2::password_hash::rand_core::OsRng;
use uuid::Uuid;

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}