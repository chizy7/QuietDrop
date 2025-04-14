use quietdrop_core::client;
use quietdrop_core::encryption::generate_keypair;
use quietdrop_core::message::{get_input, Message, MessageType};
use quietdrop_core::server;
use sodiumoxide::crypto::box_;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use tokio::runtime::Runtime;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: securechat <server|client>");
        std::process::exit(1);
    }

    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    let server_public_key_bytes;
    let server_secret_key_bytes;

    match args[1].as_str() {
        "server" => {
            // you basically generate server keypair this way
            let (public_key, secret_key) = generate_keypair();

            server_public_key_bytes = public_key.as_ref().to_vec();
            server_secret_key_bytes = secret_key.as_ref().to_vec();

            // and then you basically save server's keys to files for later use
            let _ = File::create("server_public_key.key")
                .and_then(|mut file| file.write_all(&server_public_key_bytes));
            let _ = File::create("server_secret_key.key")
                .and_then(|mut file| file.write_all(&server_secret_key_bytes));

            let server_secret_key = box_::SecretKey::from_slice(&server_secret_key_bytes).unwrap();

            println!("\n>>> Now listening for incoming messages...\n");

            // and then you run the server using its secret key this way
            rt.block_on(server::run_server("127.0.0.1:8080", &server_secret_key))
                .expect("Server failed to run");
        }
        "client" => {
            // now here, you basically generat the client keypair
            let (public_key, secret_key) = generate_keypair();

            // then read server's keys from files
            // from the file you saved --> server's public key
            let mut file =
                File::open("server_public_key.key").expect("Unable to open the key file");

            let mut server_public_key_bytes = Vec::new();
            file.read_to_end(&mut server_public_key_bytes)
                .expect("Unable to read the key file");

            let server_public_key = box_::PublicKey::from_slice(&server_public_key_bytes).unwrap();

            println!("\n");
            let name = get_input("Enter your name: ");
            let msg_str = get_input("Enter your message: ");

            let mut msg = Message {
                timestamp: chrono::Utc::now(),
                message_type: MessageType::Text,
                sender: name.clone(),
                recipient: "Bob".to_owned(),
                content: vec![],
                public_key: public_key.clone(),
            };
            msg.encrypt_content(&msg_str, &server_public_key, &secret_key);
            rt.block_on(client::send_message(&msg, "127.0.0.1:8080"))
                .expect("Client failed to send message");
        }
        _ => {
            eprintln!("Invalid argument. Use 'client' or 'server'.");
            std::process::exit(1);
        }
    }
}
