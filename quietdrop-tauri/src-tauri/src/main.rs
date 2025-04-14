#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use quietdrop_core::client;
use quietdrop_core::encryption::generate_keypair;
use quietdrop_core::message::{Message, MessageType};
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::box_;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Mutex;
use tauri::State;

#[derive(Default)]
struct AppState {
    server_address: Mutex<String>,
}

#[derive(Serialize)]
struct MessageResponse {
    status: String,
    message: String,
}

#[derive(Deserialize)]
struct MessageRequest {
    name: String,
    content: String,
    recipient: String,
}

#[tauri::command]
async fn send_message(
    app_state: State<'_, AppState>,
    message_req: MessageRequest,
) -> Result<MessageResponse, String> {
    // Log the incoming request
    println!("Received message request from: {}", message_req.name);

    // Initialize sodiumoxide
    sodiumoxide::init().expect("Failed to initialize sodiumoxide");

    // Get server address from state
    let server_addr = {
        let server_address = app_state.server_address.lock().map_err(|e| e.to_string())?;
        if server_address.is_empty() {
            "127.0.0.1:8080".to_string()
        } else {
            server_address.clone()
        }
    };
    println!("Using server address: {}", server_addr);

    // Try to read the server's public key
    println!("Attempting to read server public key...");
    let server_key_result = read_server_public_key();
    match &server_key_result {
        Ok(_) => println!("Successfully read server public key"),
        Err(e) => println!("Error reading server public key: {}", e),
    }

    let server_public_key = server_key_result?;

    // Generate client keypair
    println!("Generating client keypair...");
    let (public_key, secret_key) = generate_keypair();

    // Create and encrypt the message
    println!("Creating message...");
    let mut msg = Message {
        timestamp: chrono::Utc::now(),
        message_type: MessageType::Text,
        sender: message_req.name,
        recipient: message_req.recipient,
        content: vec![],
        public_key: public_key.clone(),
    };

    println!("Encrypting message...");
    msg.encrypt_content(&message_req.content, &server_public_key, &secret_key);

    // Send the message to the server
    println!("Sending message to server...");
    match client::send_message(&msg, &server_addr).await {
        Ok(_) => {
            println!("Message sent successfully!");
            Ok(MessageResponse {
                status: "success".to_string(),
                message: "Message sent successfully".to_string(),
            })
        }
        Err(e) => {
            println!("Failed to send message: {}", e);
            Err(format!("Failed to send message: {}", e))
        }
    }
}

fn read_server_public_key() -> Result<box_::PublicKey, String> {
    // Try to find the key in the current directory and parent directories
    let possible_paths = [
        "server_public_key.key",
        "../server_public_key.key",
        "../../server_public_key.key",
    ];

    for path in possible_paths {
        println!("Trying to open key file: {}", path);
        match File::open(path) {
            Ok(mut file) => {
                let mut server_public_key_bytes = Vec::new();
                match file.read_to_end(&mut server_public_key_bytes) {
                    Ok(_) => match box_::PublicKey::from_slice(&server_public_key_bytes) {
                        Some(key) => return Ok(key),
                        None => println!("Invalid key format in {}", path),
                    },
                    Err(e) => println!("Error reading {}: {}", path, e),
                }
            }
            Err(e) => println!("Could not open {}: {}", path, e),
        }
    }

    Err("Could not find server public key in any expected location".to_string())
}

#[tauri::command]
fn set_server_address(app_state: State<AppState>, address: String) -> Result<(), String> {
    println!("Setting server address to: {}", address);
    if let Ok(mut server_address) = app_state.server_address.lock() {
        *server_address = address;
        Ok(())
    } else {
        Err("Failed to update server address".to_string())
    }
}

fn main() {
    println!("Starting QuietDrop Tauri application...");

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            server_address: Mutex::new("127.0.0.1:8080".to_string()),
        })
        .invoke_handler(tauri::generate_handler![send_message, set_server_address])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
