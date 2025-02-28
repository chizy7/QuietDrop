<div align="center">
    <img src="https://via.placeholder.com/150x150?text=🔒" alt="QuietDrop" height="136" />
<h1 align="center">
    QuietDrop: End-to-End Encrypted Messaging in Rust
</h1>

[![CI Status][a1]][a2] [![License][b1]][b2] [![Lines of Code][c1]][c2] [![GitHub Stars][d1]][d2] [![Contributor Covenant][e1]][e2]

[a1]: https://img.shields.io/github/actions/workflow/status/chizy7/QuietDrop/ci.yml?branch=main&style=flat-square&label=build
[a2]: https://github.com/chizy7/QuietDrop/actions
[b1]: https://img.shields.io/github/license/chizy7/QuietDrop?style=flat-square
[b2]: https://github.com/chizy7/QuietDrop/blob/main/LICENSE
[c1]: https://tokei.rs/b1/github/chizy7/QuietDrop?category=code
[c2]: https://github.com/chizy7/QuietDrop
[d1]: https://img.shields.io/github/stars/chizy7/QuietDrop?style=flat-square
[d2]: https://github.com/chizy7/QuietDrop/stargazers
[e1]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg
[e2]: CODE_OF_CONDUCT.md

> <img src="https://via.placeholder.com/60x60?text=🛡️" alt="QuietDrop mascot" style="vertical-align: middle" align="left" height="60" />QuietDrop is a secure chat application built in Rust, focusing on privacy and security. It implements end-to-end encryption using modern cryptographic libraries to ensure your messages remain confidential.

<br/>

[Getting Started](docs/DETAILED_DOCS.md)&nbsp;&nbsp;•&nbsp;&nbsp;
[Security Architecture](docs/ENCRYPTION.md)&nbsp;&nbsp;•&nbsp;&nbsp;
[API Reference](docs/API.md)

<br/>
</div>

## Features

- **End-to-End Encryption**: Messages are encrypted on the sender's device and can only be decrypted by the intended recipient
- **Strong Authentication**: Secure user authentication with Argon2id password hashing
- **Forward Secrecy**: Protection of past communications even if keys are compromised
- **Group Chats**: Secure communication with multiple participants (in development)
- **File Transfer**: Encrypted file sharing between users (in development)

## Quick Start

```bash
# Clone the repository
git clone https://github.com/chizy7/QuietDrop.git
cd QuietDrop

# Build the project
cargo build --release

# Run the server
cargo run server

# In another terminal, run the client
cargo run client
```

## Usage

QuietDrop uses a client-server architecture with public key cryptography for message encryption.

```rust
// Example: Encrypting a message
let message = "Hello, secure world!";
let encrypted = encrypt_message(&message, &recipient_public_key, &sender_secret_key);

// Example: Sending a message
let msg = Message {
    timestamp: chrono::Utc::now(),
    message_type: MessageType::Text,
    sender: "Alice".to_owned(),
    recipient: "Bob".to_owned(),
    content: vec![],
    public_key: sender_public_key,
};
msg.encrypt_content(&message, &recipient_public_key, &sender_secret_key);
```

## Project Structure

```
QuietDrop/
├── src/                - Source code
│   ├── authentication.rs  - User authentication
│   ├── client.rs       - Client implementation
│   ├── encryption.rs   - Cryptographic operations
│   ├── lib.rs          - Library exports
│   ├── main.rs         - Application entry point
│   ├── message.rs      - Message handling
│   └── server.rs       - Server implementation
├── docs/               - Documentation
│   ├── DETAILED_DOCS.md  - Comprehensive documentation
│   ├── ENCRYPTION.md   - Security architecture
│   ├── API.md          - API reference
│   ├── PROJECT_ROADMAP.md - Development roadmap
│   └── SECURITY.md     - Security policy
├── tests/              - Integration tests
│   ├── authentication_test.rs - Authentication tests
│   ├── encryption_integration_tests.rs - Encryption tests
│   ├── message_test.rs - Message handling tests
│   ├── client_server_test.rs - Client-server tests
│   └── README.md       - Test documentation
├── .github/            - GitHub configuration
│   ├── ISSUE_TEMPLATE/ - Issue templates
│   └── workflows/      - CI/CD workflows
├── CODE_OF_CONDUCT.md  - Community standards
├── CONTRIBUTING.md     - Contribution guidelines
├── LICENSE             - MIT License
└── README.md           - This file
```

## Testing

QuietDrop includes a comprehensive test suite to ensure reliability:

```bash
# Run all tests
cargo test

# Run specific component tests
cargo test encryption

# Run tests with output
cargo test -- --nocapture

# Run ignored tests that require special setup
cargo test -- --ignored
```

## Contributing

QuietDrop is an active open-source project that welcomes contributions:

- Check out our [Contributing Guide](CONTRIBUTING.md)
- Browse [open issues](https://github.com/chizy7/QuietDrop/issues)
- Explore the [Project Roadmap](docs/PROJECT_ROADMAP.md)

## Security

Security is our top priority. If you discover a vulnerability, please see our [Security Policy](docs/SECURITY.md) for reporting guidelines.

## License

QuietDrop is licensed under the [MIT License](LICENSE).

## Acknowledgements

- [sodiumoxide](https://github.com/sodiumoxide/sodiumoxide) for cryptographic operations
- [rustls](https://github.com/rustls/rustls) for TLS implementation
- [Tokio](https://github.com/tokio-rs/tokio) for asynchronous runtime
- [Serde](https://github.com/serde-rs/serde) for serialization
- [Argon2](https://github.com/P-H-C/phc-winner-argon2) for password hashing