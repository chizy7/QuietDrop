use anyhow::{Result, Error}; // Import Result and Error from anyhow
use argon2::{self, Argon2, password_hash::{SaltString, PasswordHash}, PasswordHasher, PasswordVerifier};
use argon2::Algorithm;
use rand::rngs::OsRng;
use argon2::Params;
use argon2::Version;
use std::fs::File;
use std::io::{Read, Write};

const SALT_FILE: &str = "salt.txt";

pub fn save_salt(salt: &SaltString) -> std::io::Result<()> {
    let mut file = File::create(SALT_FILE)?;
    file.write_all(salt.as_ref().as_bytes())?;
    Ok(())
}

pub fn load_salt() -> Result<SaltString> {
    let mut file = File::open(SALT_FILE)?;
    let mut salt_str = String::new();
    file.read_to_string(&mut salt_str)?;
    SaltString::new(&salt_str).map_err(|e| Error::msg(e.to_string())) // Convert error into anyhow::Error
}

pub fn hash_password(password: &str) -> Result<(String, String)> {
    let salt = SaltString::generate(&mut OsRng);
    let config = Params::new(3, 4096, 1, None).map_err(|e| Error::msg(e.to_string()))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, config);
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
                             .map_err(|e| Error::msg(e.to_string()))?
                             .to_string();

    Ok((password_hash, salt.as_str().to_string()))
}

pub fn verify_password(hashed_password: &str, _salt: &str, password: &str) -> Result<bool> {
    let password_hash = PasswordHash::new(hashed_password).map_err(|e| Error::msg(e.to_string()))?;
    let config = Params::new(3, 4096, 1, None).map_err(|e| Error::msg(e.to_string()))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, config);

    Ok(argon2.verify_password(password.as_bytes(), &password_hash).is_ok())
}
