use crate::message::Message;
use serde_json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::spawn;

pub async fn run_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        spawn(async move {
            let mut buf = [0; 1024];

            // Read a message from the socket
            match socket.read(&mut buf).await {
                Ok(0) => {
                    eprintln!("Connection closed.");
                }
                Ok(n) => {
                    // Deserialize the message
                    let msg: Message = serde_json::from_slice(&buf[..n]).unwrap();
                    println!("Received: {:?}", msg);

                    // TODO: Add logic for handling messages, such as storing them, broadcasting to other clients, etc.

                    // Send an acknowledgement back to the client
                    let response = "Message received.";
                    socket.write_all(response.as_bytes()).await.unwrap();
                }
                Err(e) => {
                    eprintln!("Error reading from socket: {}", e);
                }
            }
        });
    }
}
