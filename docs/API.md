# QuietDrop API Documentation

This document describes the core API for QuietDrop, an end-to-end encrypted chat application built in Rust.

## Table of Contents

- [Overview](#overview)
- [Authentication Module](#authentication-module)
- [Encryption Module](#encryption-module)
- [Message Module](#message-module)
- [Client Module](#client-module)
- [Server Module](#server-module)
- [Error Handling](#error-handling)
- [Examples](#examples)

## Overview

QuietDrop is structured as a collection of modules, each handling specific functionality. The core APIs are exposed through these modules and can be used by developers to extend or integrate with the application.

## Authentication Module

The authentication module (`authentication.rs`) handles user authentication, password hashing, and verification.

### Key Functions

```rust
/// Hash a password using Argon2id with secure parameters
pub fn hash_password(password: &str) -> Result<(String, String)>

/// Verify a password against a stored hash
pub fn verify_password(hashed_password: &str, salt: &str, password: &str) -> Result<bool>

/// Save a salt to a file
pub fn save_salt(salt: &SaltString) -> std::io::Result<()>

/// Load a salt from a file
pub fn load_salt() -> Result<SaltString>
```

### Usage Example

```rust
use quietdrop::authentication;

// Create a new user account
let password = "secure_password123";
let (hashed_password, salt) = authentication::hash_password(password)?;

// Later, verify the password
let is_valid = authentication::verify_password(&hashed_password, &salt, password)?;
assert!(is_valid);
```

## Encryption Module

The encryption module (`encryption.rs`) provides cryptographic operations for secure messaging.

### Types

```rust
pub type PublicKey = box_::PublicKey;
pub type SecretKey = box_::SecretKey;
pub type KeyPair = (PublicKey, SecretKey);
```

### Key Functions

```rust
/// Generate a new keypair for asymmetric encryption
pub fn generate_keypair() -> KeyPair

/// Encrypt a message using the receiver's public key and sender's secret key
pub fn encrypt_message(message: &str, public_key: &PublicKey, secret_key: &SecretKey) -> Vec<u8>

/// Decrypt a message using the sender's public key and receiver's secret key
pub fn decrypt_message(
    encrypted_data: &[u8],
    public_key: &PublicKey,
    secret_key: &SecretKey,
) -> Result<String, &'static str>
```

### Usage Example

```rust
use quietdrop::encryption;

// Generate keypairs for Alice and Bob
let (alice_public_key, alice_secret_key) = encryption::generate_keypair();
let (bob_public_key, bob_secret_key) = encryption::generate_keypair();

// Alice encrypts a message for Bob
let message = "Hello, Bob! This is a secret message.";
let encrypted = encryption::encrypt_message(
    message, 
    &bob_public_key, 
    &alice_secret_key
);

// Bob decrypts the message from Alice
let decrypted = encryption::decrypt_message(
    &encrypted,
    &alice_public_key,
    &bob_secret_key
)?;

assert_eq!(message, decrypted);
```

## Message Module

The message module (`message.rs`) defines message structures and handling.

### Types

```rust
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
```

### Methods

```rust
impl Message {
    /// Encrypt the content of a message
    pub fn encrypt_content(
        &mut self,
        plaintext: &str,
        receiver_public_key: &PublicKey,
        sender_secret_key: &SecretKey,
    )
    
    // Additional methods...
}
```

### Helper Functions

```rust
/// Get user input with a prompt
pub fn get_input(prompt: &str) -> String
```

## Client Module

The client module (`client.rs`) handles client-side operations for sending messages.

### Key Functions

```rust
/// Send a message to the server
pub async fn send_message(
    message: &Message,
    server_addr: &str,
) -> Result<(), Box<dyn std::error::Error>>
```

### Usage Example

```rust
use quietdrop::{client, message::Message, encryption};
use chrono::Utc;

// Create and send a message
async fn send_example_message() -> Result<(), Box<dyn std::error::Error>> {
    let (public_key, secret_key) = encryption::generate_keypair();
    let server_public_key = // ... load server public key
    
    let mut msg = Message {
        timestamp: Utc::now(),
        message_type: MessageType::Text,
        sender: "Alice".to_owned(),
        recipient: "Bob".to_owned(),
        content: vec![],
        public_key,
    };
    
    msg.encrypt_content("Hello, Bob!", &server_public_key, &secret_key);
    client::send_message(&msg, "127.0.0.1:8080").await?;
    
    Ok(())
}
```

## Server Module

The server module (`server.rs`) handles server-side operations for receiving and processing messages.

### Key Functions

```rust
/// Run the server on the specified address with the provided secret key
pub async fn run_server(
    addr: &str,
    server_secret_key: &SecretKey,
) -> Result<(), Box<dyn std::error::Error>>
```

### Internal Types

```rust
/// Rate limiter to prevent abuse
struct RateLimiter {
    requests: HashMap<SocketAddr, (usize, Instant)>,
    time_frame: std::time::Duration,
    request_limit: usize,
}
```

## Error Handling

QuietDrop uses the `anyhow` crate for error handling, providing `Result` types for operations that can fail.

Common error patterns:

```rust
// Using the ? operator with anyhow::Result
let salt = load_salt()?;

// Converting errors from other libraries
SaltString::new(&salt_str).map_err(|e| Error::msg(e.to_string()))

// Handling errors in async code
match socket.read(&mut buf).await {
    Ok(n) => {
        // Process data
    },
    Err(e) => {
        eprintln!("Error reading from socket: {}", e);
    }
}
```

## Examples

### Complete Client-Server Communication

```rust
// Server initialization
let (server_public_key, server_secret_key) = encryption::generate_keypair();
// Save the public key for clients
std::fs::write("server_public_key.key", server_public_key.as_ref())?;

// Start the server
tokio::spawn(async move {
    server::run_server("127.0.0.1:8080", &server_secret_key).await
});

// Client communication
let (client_public_key, client_secret_key) = encryption::generate_keypair();
// Load the server's public key
let server_public_key_bytes = std::fs::read("server_public_key.key")?;
let server_public_key = PublicKey::from_slice(&server_public_key_bytes)
    .ok_or_else(|| anyhow::Error::msg("Invalid public key"))?;

// Create and send a message
let mut msg = Message {
    timestamp: Utc::now(),
    message_type: MessageType::Text,
    sender: "Alice".to_owned(),
    recipient: "Bob".to_owned(),
    content: vec![],
    public_key: client_public_key,
};
msg.encrypt_content("Hello, secure world!", &server_public_key, &client_secret_key);
client::send_message(&msg, "127.0.0.1:8080").await?;
```

---

This API documentation is a living document and will be updated as QuietDrop evolves. For the latest API details, consult the source code and inline documentation.