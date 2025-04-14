#![allow(dead_code)]
use crate::encryption::{encrypt_message, PublicKey, SecretKey};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io::{stdin, stdout, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub timestamp: DateTime<Utc>,
    pub message_type: MessageType,
    pub sender: String,
    pub recipient: String,
    pub content: Vec<u8>,
    pub public_key: PublicKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    File,
}

impl Message {
    pub fn encrypt_content(
        &mut self,
        plaintext: &str,
        receiver_public_key: &PublicKey,
        sender_secret_key: &SecretKey,
    ) {
        self.content = encrypt_message(plaintext, receiver_public_key, sender_secret_key);
    }
}

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = stdout().flush();
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");

    input.trim().to_owned()
}
