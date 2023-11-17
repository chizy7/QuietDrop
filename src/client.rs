use crate::message::Message;
use serde_json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn send_message(
    message: &Message,
    server_addr: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(server_addr).await?;

    let msg_bytes = serde_json::to_vec(message)?;
    stream.write_all(&msg_bytes).await?;

    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    let response = String::from_utf8_lossy(&buf[..n]);
    println!("Server response: {}", response);

    if response != "Message received." {
        return Err("Failed to receive proper acknowledgement from server".into())
    }

    Ok(())
}
