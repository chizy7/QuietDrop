use securechat::client;
use securechat::message::{Message, MessageType};
use securechat::server;
use std::env;
use tokio::runtime::Runtime;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: securechat <server|client>");
        std::process::exit(1);
    }

    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    match args[1].as_str() {
        "server" => {
            rt.block_on(server::run_server("127.0.0.1:8080"))
               .expect("Server failed to run");
        }
        "client" => {
            let msg = Message {
                timestamp: chrono::Utc::now(),
                message_type: MessageType::Text,
                sender: "Alice".to_string(),
                recipient: "Bob".to_string(),
                content: "Hello, Bob!".to_string(),
            };
            rt.block_on(client::send_message(&msg, "127.0.0.1:8080"))
               .expect("Client failed to send message");
        }
        _ => {
            eprintln!("Invalid argument. Use 'server' or 'client'.");
            std::process::exit(1);
        }
    }
}
