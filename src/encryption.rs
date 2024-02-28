use sodiumoxide::crypto::box_;
use std::result::Result;

pub type PublicKey = box_::PublicKey;
pub type SecretKey = box_::SecretKey;
pub type KeyPair = (PublicKey, SecretKey);

pub fn generate_keypair() -> KeyPair {
    box_::gen_keypair()
}

pub fn encrypt_message(message: &str, public_key: &PublicKey, secret_key: &SecretKey) -> Vec<u8> {
    let nonce = box_::gen_nonce();
    let encrypted_msg = box_::seal(message.as_bytes(), &nonce, public_key, secret_key);

    [nonce.as_ref(), encrypted_msg.as_slice()].concat()
}

pub fn decrypt_message(
    encrypted_data: &[u8],
    public_key: &PublicKey,
    secret_key: &SecretKey,
) -> Result<String, &'static str> {
    if encrypted_data.len() < box_::NONCEBYTES {
        return Err("Encrypted data is too short");
    }

    let (nonce_bytes, encrypted_msg) = encrypted_data.split_at(box_::NONCEBYTES);
    let nonce = box_::Nonce::from_slice(nonce_bytes).ok_or("Failed to construct nonce")?;

    let decrypted_msg = box_::open(encrypted_msg, &nonce, public_key, secret_key)
        .map_err(|_| "Decryption failed")?;

    String::from_utf8(decrypted_msg).map_err(|_| "Invalid UTF-8")
}
