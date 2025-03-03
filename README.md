<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="./.images/quietdrop-logo-white.svg">
    <source media="(prefers-color-scheme: light)" srcset="./.images/quietdrop-logo-black.svg">
    <img alt="QuietDrop Logo" src="./.images/quietdrop-logo-black.svg" width="550" height="200">
  </picture>
  <h1>QuietDrop: End-to-End Encrypted Messaging in Rust</h1>
  <p>
    <a href="https://github.com/chizy7/QuietDrop/actions"><img src="https://img.shields.io/github/actions/workflow/status/chizy7/QuietDrop/ci.yml?branch=main&style=flat-square&label=build" alt="Build Status"></a>
    <a href="https://github.com/chizy7/QuietDrop/blob/main/LICENSE"><img src="https://img.shields.io/github/license/chizy7/QuietDrop?style=flat-square" alt="License"></a>
    <a href="https://github.com/chizy7/QuietDrop"><img src="https://tokei.rs/b1/github/chizy7/QuietDrop?category=code" alt="Lines of Code"></a>
    <a href="https://github.com/chizy7/QuietDrop/stargazers"><img src="https://img.shields.io/github/stars/chizy7/QuietDrop?style=flat-square" alt="GitHub Stars"></a>
    <a href="CODE_OF_CONDUCT.md"><img src="https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg" alt="Contributor Covenant"></a>
    <a href="https://github.com/chizy7/QuietDrop/pulls"><img src="https://img.shields.io/coderabbit/prs/github/chizy7/QuietDrop" alt="CodeRabbit Pull Request Reviews"></a>
  </p>
</div> 

> <picture>
>   <source media="(prefers-color-scheme: dark)" srcset="./.images/quietdrop-icon-white.svg">
>   <source media="(prefers-color-scheme: light)" srcset="./.images/quietdrop-icon-black.svg">
>   <img alt="QuietDrop mascot" src="./.images/quietdrop-icon-black.svg" style="vertical-align: middle; margin-right: 12px" align="left" height="60">
> </picture>
> QuietDrop is a secure chat application built in Rust, focusing on privacy and security. It implements end-to-end encryption using modern cryptographic libraries to ensure your messages remain confidential.

<br/>

<div align="center">
  <p>
    <a href="docs/DETAILED_DOCS.md">Getting Started</a>&nbsp;&nbsp;•&nbsp;&nbsp;
    <a href="docs/ENCRYPTION.md">Security Architecture</a>&nbsp;&nbsp;•&nbsp;&nbsp;
    <a href="docs/API.md">API Reference</a>
  </p>
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