use chrono::Utc;
use quietdrop::encryption::generate_keypair;
use quietdrop::message::{Message, MessageType};

#[test]
fn test_client_message_construction() {
    // Generate key pairs
    let (client_public_key, client_secret_key) = generate_keypair();
    let (server_public_key, _) = generate_keypair();

    // Create a message
    let mut msg = Message {
        timestamp: Utc::now(),
        message_type: MessageType::Text,
        sender: "TestClient".to_owned(),
        recipient: "TestRecipient".to_owned(),
        content: vec![],
        public_key: client_public_key,
    };

    // Add encrypted content
    let test_message = "Hello, server!";
    msg.encrypt_content(test_message, &server_public_key, &client_secret_key);

    // Verify message is properly constructed
    assert_eq!(msg.sender, "TestClient");
    assert_eq!(msg.recipient, "TestRecipient");
    assert!(!msg.content.is_empty());

    // Check that the message type is correct
    match msg.message_type {
        MessageType::Text => {} // Expected
        _ => panic!("Expected Text message type"),
    }

    // Verify timestamp is reasonable (within the last minute)
    let now = Utc::now();
    let diff = now.signed_duration_since(msg.timestamp);
    assert!(diff.num_seconds() < 60, "Timestamp should be recent");
}

// The following tests would involve actual client-server communication
// They are ignored by default as they require running a server
// You can run them with: cargo test -- --ignored

#[test]
#[ignore]
fn test_server_initialization() {
    // This would test that the server can initialize and bind to a port
    // Requires tokio runtime setup
}

#[test]
#[ignore]
fn test_client_server_communication() {
    // This would test full client-server message exchange
    // Requires both client and server components running
}
