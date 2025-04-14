#![allow(dead_code)]
use crate::encryption::{decrypt_message, SecretKey};
use crate::message::Message;
use serde_json;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Instant;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::spawn;

struct RateLimiter {
    requests: HashMap<SocketAddr, (usize, Instant)>,
    time_frame: std::time::Duration,
    request_limit: usize,
}

impl RateLimiter {
    pub fn new(time_frame: std::time::Duration, request_limit: usize) -> Self {
        RateLimiter {
            requests: HashMap::new(),
            time_frame,
            request_limit,
        }
    }

    pub fn check(&mut self, addr: SocketAddr) -> bool {
        let (count, timestamp) = self.requests.entry(addr).or_insert((0, Instant::now()));
        if timestamp.elapsed() > self.time_frame {
            *count = 0;
            *timestamp = Instant::now();
        }
        if *count > self.request_limit {
            false
        } else {
            *count += 1;
            true
        }
    }
}

pub async fn run_server(
    addr: &str,
    server_secret_key: &SecretKey,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        let server_secret_key = server_secret_key.clone();

        spawn(async move {
            let mut buf = vec![0; 1024];
            match socket.read(&mut buf).await {
                Ok(0) => {
                    eprintln!("Connection closed.");
                }

                Ok(n) => {
                    match serde_json::from_slice::<Message>(&buf[..n]) {
                        Ok(msg) => {
                            println!("### Encrypted message recived:\n{:?}\n", &msg.content);

                            // And then you get the sender's public key here
                            let sender_public_key = msg.public_key;

                            // Then you decrypt using the secret key this way
                            match decrypt_message(
                                &msg.content,
                                &sender_public_key,
                                &server_secret_key,
                            ) {
                                Ok(content) => {
                                    println!(
                                        "## Decrypted message: \n\
                                        sender: {}\n\
                                        recipient: {}\n\
                                        content: {}\n\
                                        timestamp: {}\n",
                                        msg.sender, msg.recipient, content, msg.timestamp,
                                    );
                                    let response = "Message received.";
                                    if let Err(e) = socket.write_all(response.as_bytes()).await {
                                        eprintln!("Failed to send response: {}", e);
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error decrypting message: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error deserializing message: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading from socket: {}", e);
                }
            }
        });
    }
}
