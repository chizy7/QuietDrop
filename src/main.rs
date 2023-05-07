use securechat::client;
use securechat::message::Message;
use securechat::server;
use std::env;
use tokio::runtime::Runtime;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: securechat <server|client>");
        return;
    }

    let rt = Runtime::new().unwrap();
    match args[1].as_str() {
        "server" => {
            rt.block_on(server::run_server("127.0.0.1:8080")).unwrap();
        }
        "client" => {
            let msg = Message {
                sender: "Alice".to_string(),
                recipient: "Bob".to_string(),
                content: "Hello, Bob!".to_string(),
            };
            rt.block_on(client::send_message(&msg, "127.0.0.1:8080")).unwrap();
        }
        _ => {
            eprintln!("Invalid argument. Use 'server' or 'client'.");
        }
    }
}
