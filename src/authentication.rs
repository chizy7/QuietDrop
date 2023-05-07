use argon2::{self, Argon2, password_hash::{SaltString, PasswordHash, PasswordHasher, PasswordVerifier}};
use rand::rngs::OsRng;

pub fn hash_password(password: &str) -> (String, String) {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password_simple(password.as_bytes(), salt.as_ref())
        .unwrap()
        .to_string();

    (password_hash, salt.to_string())
}

pub fn verify_password(hashed_password: &str, salt: &str, password: &str) -> bool {
    let password_hash = PasswordHash::new(hashed_password).unwrap();
    let salt_string = SaltString::new(salt).unwrap();

    let argon2 = Argon2::default();
    argon2.verify_password(password.as_bytes(), &password_hash).is_ok()
}
