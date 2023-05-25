// use argon2::{self, Argon2, password_hash::{SaltString, PasswordHash, PasswordHasher, PasswordVerifier}};
// use rand::rngs::OsRng;

// pub fn hash_password(password: &str) -> (String, String) {
//     let salt = SaltString::generate(&mut OsRng);

//     let argon2 = Argon2::default();
//     let password_hash = argon2.hash_password_simple(password.as_bytes(), salt.as_ref())
//         .unwrap()
//         .to_string();

//     (password_hash, salt.to_string())
// }

// pub fn verify_password(hashed_password: &str, salt: &str, password: &str) -> bool {
//     let password_hash = PasswordHash::new(hashed_password).unwrap();
//     let salt_string = SaltString::new(salt).unwrap();

//     let argon2 = Argon2::default();
//     argon2.verify_password(password.as_bytes(), &password_hash).is_ok()
// }

use argon2::{self, Argon2, password_hash::{SaltString, PasswordHash}, PasswordHasher, PasswordVerifier};
use argon2::Algorithm;
use rand::rngs::OsRng;
use argon2::Params;
use argon2::Version;

pub fn hash_password(password: &str) -> (String, String) {
    let salt = SaltString::generate(&mut OsRng);
    let config = Params::new(3, 4096, 1, None).unwrap();

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, config);
    let password_hash = argon2.hash_password(password.as_bytes(), salt.as_ref())
        .unwrap()
        .to_string();

    (password_hash, salt.as_str().to_string())
}

pub fn verify_password(hashed_password: &str, salt: &str, password: &str) -> bool {
    let password_hash = PasswordHash::new(hashed_password).unwrap();
    let salt_string = SaltString::new(salt).unwrap();

    let config = Params::new(3, 4096, 1, None).unwrap();
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, config);

    argon2.verify_password(password.as_bytes(), &password_hash).is_ok()
}
