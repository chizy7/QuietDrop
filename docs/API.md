# QuietDrop API Documentation

This document describes the core API for QuietDrop, an end-to-end encrypted messaging application built in Rust with Tauri.

## Table of Contents

- [Overview](#overview)
- [Core Library API](#core-library-api)
  - [Authentication Module](#authentication-module)
  - [Encryption Module](#encryption-module)
  - [Message Module](#message-module)
  - [Client Module](#client-module)
  - [Server Module](#server-module)
- [Tauri Commands API](#tauri-commands-api)
  - [Authentication Commands](#authentication-commands)
  - [Messaging Commands](#messaging-commands)
  - [Settings Commands](#settings-commands)
- [Frontend API](#frontend-api)
  - [Yew Components](#yew-components)
  - [State Management](#state-management)
- [Error Handling](#error-handling)
- [Examples](#examples)

## Overview

QuietDrop is structured as a Rust workspace with multiple crates:

1. **quietdrop-core**: The core library with the primary functionality
2. **quietdrop-cli**: Command-line interface using the core library
3. **quietdrop-tauri**: Desktop application using Tauri and Yew

The API is designed to be modular, allowing different interfaces to use the same core functionality.

## Core Library API

The core library (`quietdrop-core`) provides the foundation for all QuietDrop implementations.

### Authentication Module

Handles user authentication, password hashing, and verification.

#### Key Functions

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

#### Usage Example

```rust
use quietdrop_core::authentication;

// Create a new user account
let password = "secure_password123";
let (hashed_password, salt) = authentication::hash_password(password)?;

// Later, verify the password
let is_valid = authentication::verify_password(&hashed_password, &salt, password)?;
assert!(is_valid);
```

### Encryption Module

Provides cryptographic operations for secure messaging.

#### Types

```rust
pub type PublicKey = box_::PublicKey;
pub type SecretKey = box_::SecretKey;
pub type KeyPair = (PublicKey, SecretKey);
```

#### Key Functions

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

#### Usage Example

```rust
use quietdrop_core::encryption;

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

### Message Module

Defines message structures and handling.

#### Types

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

#### Methods

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

#### Helper Functions

```rust
/// Get user input with a prompt
pub fn get_input(prompt: &str) -> String
```

### Client Module

Handles client-side operations for sending messages.

#### Key Functions

```rust
/// Send a message to the server
pub async fn send_message(
    message: &Message,
    server_addr: &str,
) -> Result<(), Box<dyn std::error::Error>>
```

### Server Module

Handles server-side operations for receiving and processing messages.

#### Key Functions

```rust
/// Run the server on the specified address with the provided secret key
pub async fn run_server(
    addr: &str,
    server_secret_key: &SecretKey,
) -> Result<(), Box<dyn std::error::Error>>
```

## Tauri Commands API

The Tauri backend exposes commands that bridge between the frontend and the core library.

### Authentication Commands

```rust
#[tauri::command]
async fn login(
    app_state: State<'_, AppState>,
    username: String,
    password: String,
) -> Result<UserResponse, String> {
    // Implementation using core authentication functions
}

#[tauri::command]
async fn create_account(
    app_state: State<'_, AppState>,
    username: String,
    password: String,
) -> Result<UserResponse, String> {
    // Implementation using core authentication functions
}

#[tauri::command]
async fn logout(
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    // Implementation
}
```

### Messaging Commands

```rust
#[tauri::command]
async fn send_message(
    app_state: State<'_, AppState>,
    message_req: MessageRequest,
) -> Result<MessageResponse, String> {
    // Implementation using core client functions
}

#[tauri::command]
async fn get_messages(
    app_state: State<'_, AppState>,
) -> Result<Vec<MessageResponse>, String> {
    // Implementation
}

#[tauri::command]
async fn start_server(
    app_state: State<'_, AppState>,
    address: String,
) -> Result<(), String> {
    // Implementation using core server functions
}
```

### Settings Commands

```rust
#[tauri::command]
async fn generate_keypair(
    app_state: State<'_, AppState>,
) -> Result<KeyPairResponse, String> {
    // Implementation using core encryption functions
}

#[tauri::command]
async fn save_settings(
    app_state: State<'_, AppState>,
    settings: SettingsRequest,
) -> Result<(), String> {
    // Implementation
}

#[tauri::command]
async fn get_settings(
    app_state: State<'_, AppState>,
) -> Result<SettingsResponse, String> {
    // Implementation
}
```

## Frontend API

The Yew frontend provides a user interface for interacting with the application.

### Yew Components

#### Main Application Component

```rust
#[function_component(App)]
fn app() -> Html {
    // Component implementation
    html! {
        <div class="app">
            <Header />
            <Router>
                <Switch<Route> render={switch} />
            </Router>
        </div>
    }
}
```

#### MessageList Component

```rust
#[derive(Properties, PartialEq)]
pub struct MessageListProps {
    pub messages: Vec<Message>,
}

#[function_component(MessageList)]
pub fn message_list(props: &MessageListProps) -> Html {
    // Component implementation
}
```

#### MessageInput Component

```rust
#[function_component(MessageInput)]
pub fn message_input() -> Html {
    let message = use_state(|| String::new());
    let recipient = use_state(|| String::new());
    
    // Component implementation
}
```

### State Management

Yew's state management patterns are used to maintain application state:

```rust
// Using use_state hook
let messages = use_state(|| Vec::<Message>::new());

// Using context for global state
let app_context = use_context::<AppContext>().expect("No app context found");

// Using reducers for complex state
let state = use_reducer(|| AppState {
    // Initial state
});
```

## Error Handling

QuietDrop uses structured error handling across all components:

### Core Library Errors

```rust
// Using anyhow for general error handling
use anyhow::{Error, Result};

pub fn some_function() -> Result<T> {
    // Function implementation
    // Return Ok(value) or Err(Error::msg("error message"))
}
```

### Tauri Command Errors

```rust
#[tauri::command]
async fn some_command() -> Result<T, String> {
    // Convert core errors to String for Tauri
    core_function().map_err(|e| e.to_string())
}
```

### Frontend Error Handling

```rust
// Using Result with web-sys console for logging
fn handle_error<T>(result: Result<T, String>) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(e) => {
            web_sys::console::error_1(&e.into());
            None
        }
    }
}
```

## Examples

### Complete Tauri Application Flow

```rust
// ----- Tauri Command -----
#[tauri::command]
async fn send_message(
    app_state: State<'_, AppState>,
    message_req: MessageRequest,
) -> Result<MessageResponse, String> {
    // Get the user's keypair
    let keys = app_state.keys.lock().map_err(|e| e.to_string())?;
    let (public_key, secret_key) = keys.as_ref()
        .ok_or_else(|| "No keys found".to_string())?;
    
    // Get server public key
    let server_public_key = app_state.server_key.lock().map_err(|e| e.to_string())?;
    let server_public_key = server_public_key.as_ref()
        .ok_or_else(|| "No server key found".to_string())?;
        
    // Create and encrypt the message
    let mut message = Message {
        timestamp: Utc::now(),
        message_type: MessageType::Text,
        sender: message_req.sender,
        recipient: message_req.recipient,
        content: vec![],
        public_key: public_key.clone(),
    };
    
    message.encrypt_content(
        &message_req.content,
        server_public_key,
        secret_key
    );
    
    // Send the message
    let server_addr = app_state.server_addr.lock().map_err(|e| e.to_string())?;
    client::send_message(&message, &server_addr)
        .await
        .map_err(|e| e.to_string())?;
    
    // Return a response
    Ok(MessageResponse {
        id: uuid::Uuid::new_v4().to_string(),
        content: message_req.content,
        sender: message.sender,
        recipient: message.recipient,
        timestamp: message.timestamp,
        status: "sent".to_string(),
    })
}

// ----- Yew Frontend Component -----
#[function_component(MessageForm)]
fn message_form() -> Html {
    let content = use_state(|| String::new());
    let recipient = use_state(|| String::new());
    let status = use_state(|| String::new());
    
    let content_clone = content.clone();
    let recipient_clone = recipient.clone();
    let status_clone = status.clone();
    
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let content = (*content_clone).clone();
        let recipient = (*recipient_clone).clone();
        
        // Create the request
        let message_req = MessageRequest {
            content,
            recipient,
            sender: "current_user".to_string(),
        };
        
        // Clone states for the async block
        let status = status_clone.clone();
        
        // Send the message using Tauri command
        wasm_bindgen_futures::spawn_local(async move {
            status.set("Sending...".to_string());
            
            let result = invoke::<_, MessageResponse>(
                "send_message",
                &JsValue::from_serde(&message_req).unwrap()
            ).await;
            
            match result {
                Ok(_) => {
                    status.set("Message sent successfully".to_string());
                    content_clone.set(String::new());
                    recipient_clone.set(String::new());
                },
                Err(e) => {
                    status.set(format!("Error: {}", e));
                }
            }
        });
    });
    
    let oninput_content = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_dyn_into::<web_sys::HtmlInputElement>();
            if let Some(input) = target {
                content.set(input.value());
            }
        })
    };
    
    let oninput_recipient = {
        let recipient = recipient.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_dyn_into::<web_sys::HtmlInputElement>();
            if let Some(input) = target {
                recipient.set(input.value());
            }
        })
    };
    
    html! {
        <form onsubmit={onsubmit} class="message-form">
            <div class="form-group">
                <label for="recipient">{"Recipient:"}</label>
                <input 
                    type="text" 
                    id="recipient" 
                    value={(*recipient).clone()} 
                    oninput={oninput_recipient} 
                />
            </div>
            <div class="form-group">
                <label for="content">{"Message:"}</label>
                <textarea 
                    id="content" 
                    value={(*content).clone()} 
                    oninput={oninput_content} 
                />
            </div>
            <button type="submit">{"Send"}</button>
            <div class="status">{(*status).clone()}</div>
        </form>
    }
}
```

### CLI Usage Example

The CLI remains compatible with the core library:

```rust
use quietdrop_core::client;
use quietdrop_core::encryption::generate_keypair;
use quietdrop_core::message::{Message, MessageType};
use chrono::Utc;
use std::env;
use tokio::runtime::Runtime;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: quietdrop-cli <server|client>");
        std::process::exit(1);
    }

    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    match args[1].as_str() {
        "server" => {
            // Generate server keypair
            let (public_key, secret_key) = generate_keypair();
            
            // Save keys to disk
            // ...
            
            // Run server
            rt.block_on(server::run_server("127.0.0.1:8080", &secret_key))
                .expect("Server failed to run");
        }
        "client" => {
            // Generate client keypair
            let (public_key, secret_key) = generate_keypair();
            
            // Load server public key
            // ...
            
            // Get user input
            // ...
            
            // Create and send message
            let mut msg = Message {
                timestamp: Utc::now(),
                message_type: MessageType::Text,
                sender: "CLI User".to_owned(),
                recipient: "Server".to_owned(),
                content: vec![],
                public_key,
            };
            
            msg.encrypt_content(&message_content, &server_public_key, &secret_key);
            
            rt.block_on(client::send_message(&msg, "127.0.0.1:8080"))
                .expect("Failed to send message");
        }
        _ => {
            eprintln!("Invalid command. Use 'server' or 'client'.");
            std::process::exit(1);
        }
    }
}
```

## Integration with Other Systems

### Using QuietDrop Core in Other Applications

The `quietdrop-core` library can be easily integrated into other Rust applications:

```rust
// In your Cargo.toml
[dependencies]
quietdrop-core = { path = "../path/to/quietdrop-core", version = "0.1.0" }

// In your code
use quietdrop_core::encryption;
use quietdrop_core::message::{Message, MessageType};

fn main() {
    // Use QuietDrop functionality
    let (public_key, secret_key) = encryption::generate_keypair();
    // ...
}
```

### Event-Based Communication with Tauri

The Tauri application uses event-based communication between frontend and backend:

```rust
// Listen for events in Tauri backend
#[tauri::command]
fn listen_for_messages(window: Window) {
    // Start a background task
    std::thread::spawn(move || {
        loop {
            // Check for new messages
            if let Some(message) = check_for_new_messages() {
                // Emit event to frontend
                window.emit("new_message", message).unwrap();
            }
            
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    });
}

// In Yew frontend
fn setup_event_listeners() -> Listener {
    let messages = use_state(|| Vec::<Message>::new());
    
    let messages_clone = messages.clone();
    let unlisten = listen_to_event("new_message", move |event: Event<Message>| {
        let mut current_messages = (*messages_clone).clone();
        current_messages.push(event.payload);
        messages_clone.set(current_messages);
    });
    
    unlisten
}
```

---

This API documentation is a living document and will be updated as QuietDrop evolves. For the latest API details, consult the source code and inline documentation.