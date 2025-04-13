use chrono::Utc;
use quietdrop::encryption::generate_keypair;
use quietdrop::message::{Message, MessageType};
use serde_json;

#[test]
fn test_message_creation_and_encryption() {
    // Generate key pairs for sender and receiver
    let (sender_public_key, sender_secret_key) = generate_keypair();
    let (receiver_public_key, _receiver_secret_key) = generate_keypair();

    // Create a message
    let mut message = Message {
        timestamp: Utc::now(),
        message_type: MessageType::Text,
        sender: "Alice".to_owned(),
        recipient: "Bob".to_owned(),
        content: vec![],
        public_key: sender_public_key.clone(),
    };

    // Original message content
    let original_content = "Hello, this is a test message";

    // Encrypt the message content
    message.encrypt_content(original_content, &receiver_public_key, &sender_secret_key);

    // Verify the content is not empty after encryption
    assert!(
        !message.content.is_empty(),
        "Encrypted content should not be empty"
    );

    // Verify the content is not the same as the original
    assert_ne!(
        message.content,
        original_content.as_bytes().to_vec(),
        "Encrypted content should differ from original"
    );
}

#[test]
fn test_message_serialization() {
    // Generate a keypair for the message
    let (public_key, secret_key) = generate_keypair();

    // Create a message
    let mut message = Message {
        timestamp: Utc::now(),
        message_type: MessageType::Text,
        sender: "TestSender".to_owned(),
        recipient: "TestRecipient".to_owned(),
        content: vec![],
        public_key: public_key.clone(),
    };

    // Add some content
    message.encrypt_content("Test content", &public_key, &secret_key);

    // Serialize to JSON
    let serialized = serde_json::to_string(&message).expect("Message should serialize to JSON");

    // Verify serialization produced something
    assert!(
        !serialized.is_empty(),
        "Serialized message should not be empty"
    );

    // Deserialize back to a message
    let deserialized: Message = serde_json::from_str(&serialized)
        .expect("Serialized message should deserialize back to Message");

    // Verify fields match (except message_type which we can't compare directly)
    assert_eq!(message.sender, deserialized.sender);
    assert_eq!(message.recipient, deserialized.recipient);
    assert_eq!(message.content, deserialized.content);

    // Check message_type manually using pattern matching instead of direct comparison
    match (message.message_type, deserialized.message_type) {
        (MessageType::Text, MessageType::Text) => {}
        (MessageType::File, MessageType::File) => {}
        _ => panic!("Message types don't match after serialization/deserialization"),
    }

    // Public key should also serialize/deserialize correctly
    assert_eq!(
        message.public_key.as_ref(),
        deserialized.public_key.as_ref()
    );
}

#[test]
fn test_message_types() {
    // Create messages of different types and check serialization

    // First verify that MessageType::Text and MessageType::File are distinct
    let is_text = matches!(MessageType::Text, MessageType::Text);
    let is_file = matches!(MessageType::File, MessageType::File);
    assert!(is_text, "MessageType::Text should match itself");
    assert!(is_file, "MessageType::File should match itself");
    assert!(
        !matches!(MessageType::Text, MessageType::File),
        "MessageType variants should be distinct"
    );

    // Verify message types serialize correctly
    let text_json =
        serde_json::to_string(&MessageType::Text).expect("MessageType::Text should serialize");
    let file_json =
        serde_json::to_string(&MessageType::File).expect("MessageType::File should serialize");

    assert_ne!(
        text_json, file_json,
        "Serialized message types should be different"
    );

    // Verify deserializing works
    let deserialized_text: MessageType =
        serde_json::from_str(&text_json).expect("Should deserialize text message type");
    let deserialized_file: MessageType =
        serde_json::from_str(&file_json).expect("Should deserialize file message type");

    match deserialized_text {
        MessageType::Text => {} // This is expected
        _ => panic!("Expected Text message type"),
    }

    match deserialized_file {
        MessageType::File => {} // This is expected
        _ => panic!("Expected File message type"),
    }

    // Test that message types are preserved in message structures
    let text_message = Message {
        timestamp: Utc::now(),
        message_type: MessageType::Text,
        sender: "Sender".to_owned(),
        recipient: "Recipient".to_owned(),
        content: vec![1, 2, 3, 4], // Dummy content
        public_key: generate_keypair().0,
    };

    let file_message = Message {
        timestamp: Utc::now(),
        message_type: MessageType::File,
        sender: "Sender".to_owned(),
        recipient: "Recipient".to_owned(),
        content: vec![1, 2, 3, 4], // Dummy content
        public_key: generate_keypair().0,
    };

    // Verify message types in structures using pattern matching
    assert!(matches!(text_message.message_type, MessageType::Text));
    assert!(matches!(file_message.message_type, MessageType::File));
}
