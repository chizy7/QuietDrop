use quietdrop::encryption::{decrypt_message, encrypt_message, generate_keypair};

#[test]
fn test_end_to_end_encryption() {
    // Generate key pairs for two users (Alice and Bob)
    let (alice_public_key, alice_secret_key) = generate_keypair();
    let (bob_public_key, bob_secret_key) = generate_keypair();

    // Alice creates a message for Bob
    let original_message = "Hello Bob, this is a secure message from Alice!";

    // Alice encrypts the message using Bob's public key and her own secret key
    let encrypted_message = encrypt_message(original_message, &bob_public_key, &alice_secret_key);

    // Verify that the encrypted message is different from the original
    assert_ne!(
        encrypted_message,
        original_message.as_bytes().to_vec(),
        "Encrypted message should be different from original"
    );

    // Bob decrypts the message using Alice's public key and his own secret key
    let decrypted_message = decrypt_message(&encrypted_message, &alice_public_key, &bob_secret_key)
        .expect("Decryption should succeed");

    // Verify Bob can read the original message
    assert_eq!(
        original_message, decrypted_message,
        "Decrypted message should match the original"
    );

    // Eve (an attacker) tries to decrypt the message with her own keys
    let (_eve_public_key, eve_secret_key) = generate_keypair();
    let eve_decryption_result =
        decrypt_message(&encrypted_message, &alice_public_key, &eve_secret_key);

    // Verify Eve cannot decrypt the message
    assert!(
        eve_decryption_result.is_err(),
        "Eve should not be able to decrypt the message"
    );
}

#[test]
fn test_message_tampering_detection() {
    // Generate key pairs
    let (alice_public_key, alice_secret_key) = generate_keypair();
    let (bob_public_key, bob_secret_key) = generate_keypair();

    // Original message
    let original_message = "This message integrity must be protected";

    // Encrypt the message
    let mut encrypted_message =
        encrypt_message(original_message, &bob_public_key, &alice_secret_key);

    // Tamper with the encrypted data (modify the last byte)
    if let Some(last) = encrypted_message.last_mut() {
        *last ^= 0x01; // Flip one bit
    }

    // Attempt to decrypt the tampered message
    let decryption_result = decrypt_message(&encrypted_message, &alice_public_key, &bob_secret_key);

    // Verify that tampering is detected
    assert!(
        decryption_result.is_err(),
        "Tampered message should fail authentication"
    );
}

#[test]
/// Tests that the encryption system enforces key specificity.
///
/// This test generates key pairs for three users: Alice, Bob, and Charlie. It verifies that
/// a message encrypted for a specific recipient can only be decrypted by that recipient:
/// - Bob can decrypt the message intended for him but fails to decrypt Charlie's message.
/// - Charlie can decrypt his own message but fails when attempting to decrypt Bob's message.
///
/// # Examples
///
/// ```
/// // Run the key specificity test.
/// test_key_specificity();
/// ```
fn test_key_specificity() {
    // Generate key pairs for three users
    let (alice_public_key, alice_secret_key) = generate_keypair();
    let (bob_public_key, bob_secret_key) = generate_keypair();
    let (charlie_public_key, charlie_secret_key) = generate_keypair();

    // Alice sends a message to Bob
    let message_for_bob = "Hey Bob, this is for your eyes only";
    let encrypted_for_bob = encrypt_message(message_for_bob, &bob_public_key, &alice_secret_key);

    // Alice sends a different message to Charlie
    let message_for_charlie = "Hey Charlie, here's the info you requested";
    let encrypted_for_charlie =
        encrypt_message(message_for_charlie, &charlie_public_key, &alice_secret_key);

    // Bob should be able to decrypt his message
    let bob_decryption = decrypt_message(&encrypted_for_bob, &alice_public_key, &bob_secret_key)
        .expect("Bob should be able to decrypt his message");
    assert_eq!(message_for_bob, bob_decryption);

    // Charlie should be able to decrypt his message
    let charlie_decryption = decrypt_message(
        &encrypted_for_charlie,
        &alice_public_key,
        &charlie_secret_key,
    )
    .expect("Charlie should be able to decrypt his message");
    assert_eq!(message_for_charlie, charlie_decryption);

    // Bob shouldn't be able to decrypt Charlie's message
    let bob_decrypting_charlies_message =
        decrypt_message(&encrypted_for_charlie, &alice_public_key, &bob_secret_key);
    assert!(bob_decrypting_charlies_message.is_err());

    // Charlie shouldn't be able to decrypt Bob's message
    let charlie_decrypting_bobs_message =
        decrypt_message(&encrypted_for_bob, &alice_public_key, &charlie_secret_key);
    assert!(charlie_decrypting_bobs_message.is_err());
}
