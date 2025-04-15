<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="./.images/quietdrop-logo-white.svg">
    <source media="(prefers-color-scheme: light)" srcset="./.images/quietdrop-logo-black.svg">
    <img alt="QuietDrop Logo" src="./.images/quietdrop-logo-black.svg" width="550" height="200">
  </picture>
  <h1>QuietDrop: End-to-End Encrypted Messaging in Rust</h1>
  <p>
    <a href="https://github.com/chizy7/QuietDrop/actions"><img src="https://img.shields.io/github/actions/workflow/status/chizy7/QuietDrop/ci.yml?branch=master&style=flat-square&label=build" alt="Build Status"></a>
    <a href="https://github.com/chizy7/QuietDrop/blob/master/LICENSE"><img src="https://img.shields.io/github/license/chizy7/QuietDrop?style=flat-square" alt="License"></a>
    <a href="https://github.com/chizy7/QuietDrop"><img src="https://img.shields.io/github/forks/chizy7/QuietDrop?style=flat-square" alt="GitHub Forks"></a>
    <a href="https://github.com/chizy7/QuietDrop/stargazers"><img src="https://img.shields.io/github/stars/chizy7/QuietDrop?style=flat-square" alt="GitHub Stars"></a>
    <a href="CODE_OF_CONDUCT.md"><img src="https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg" alt="Contributor Covenant"></a>
    <a href="https://tauri.app"><img src="https://img.shields.io/badge/Tauri-2.x-blue?style=flat-square" alt="Tauri"></a>
    <a href="https://yew.rs"><img src="https://img.shields.io/badge/Yew-0.20-orange?style=flat-square" alt="Yew"></a>
    <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/Rust-1.70%2B-dea584?style=flat-square" alt="Rust"></a>
  </p>
</div> 

> <picture>
>   <source media="(prefers-color-scheme: dark)" srcset="./.images/quietdrop-icon-white.svg">
>   <source media="(prefers-color-scheme: light)" srcset="./.images/quietdrop-icon-black.svg">
>   <img alt="QuietDrop mascot" src="./.images/quietdrop-icon-black.svg" style="vertical-align: middle; margin-right: 12px" align="left" height="60">
> </picture>
> QuietDrop is a secure messaging application built in Rust, focusing on privacy and security. It provides end-to-end encryption using modern cryptographic libraries to ensure your messages remain confidential. Available as both a command-line tool and a cross-platform application for desktop and mobile devices.

<br/>

<div align="center">
  <p>
    <a href="docs/DETAILED_DOCS.md">Getting Started</a>&nbsp;&nbsp;•&nbsp;&nbsp;
    <a href="docs/ENCRYPTION.md">Security Architecture</a>&nbsp;&nbsp;•&nbsp;&nbsp;
    <a href="docs/API.md">API Reference</a>&nbsp;&nbsp;•&nbsp;&nbsp;
    <a href="docs/TAURI_ARCHITECTURE.md">Tauri Architecture</a>
  </p>
</div>

## Features

- **End-to-End Encryption**: Messages are encrypted on the sender's device and can only be decrypted by the intended recipient
- **Cross-Platform Application**: Modern GUI application built with Tauri 2.0 for desktop and mobile
- **Command-Line Interface**: Traditional CLI for server and client operations
- **Strong Authentication**: Secure user authentication with Argon2id password hashing
- **Forward Secrecy**: Protection of past communications even if keys are compromised
- **Group Chats**: Secure communication with multiple participants (in development)
- **File Transfer**: Encrypted file sharing between users (in development)

## Quick Start

### Desktop/Mobile Application

```bash
# Clone the repository
git clone https://github.com/chizy7/QuietDrop.git
cd QuietDrop

# Build the Tauri desktop application
cd quietdrop-tauri
trunk build
cd src-tauri
cargo tauri build

# Run the application
cargo tauri dev
```

### Command Line Interface

```bash
# Clone the repository
git clone https://github.com/chizy7/QuietDrop.git
cd QuietDrop

# Build the CLI
cargo build -p quietdrop-cli --release

# Run the server
./target/release/quietdrop-cli server

# In another terminal, run the client
./target/release/quietdrop-cli client
```

## Repository Structure

| Directory | Description |
|-----------|-------------|
| `quietdrop-core/` | Core library with shared encryption and messaging functionality |
| `quietdrop-cli/` | Command-line interface for server and client operations |
| `quietdrop-tauri/` | Cross-platform application built with Tauri 2.0 and Yew |
| `quietdrop-tauri/src/` | Yew-based frontend (compiled to WebAssembly) |
| `quietdrop-tauri/src-tauri/` | Tauri backend bridge to core library |
| `docs/` | Project documentation |
| `tests/` | Integration tests for core components |
| `.github/` | GitHub configuration and CI/CD workflows |

## Usage

QuietDrop provides both a desktop/mobile interface and a command-line interface:

### Desktop/Mobile Application

The application offers an intuitive GUI for sending and receiving encrypted messages:

1. **Launch the app**: Run the QuietDrop application on your desktop or mobile device
2. **Create or load keypair**: Generate a new keypair or load an existing one
3. **Connect to server**: Enter the server address to connect
4. **Send messages**: Type your message and select a recipient
5. **View conversations**: See your message history in the conversation view

### Command Line Interface

QuietDrop's CLI uses a client-server architecture with public key cryptography:

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

## Documentation

- [Detailed Documentation](docs/DETAILED_DOCS.md): Comprehensive guide to get started
- [Encryption Architecture](docs/ENCRYPTION.md): In-depth explanation of cryptographic mechanisms
- [API Reference](docs/API.md): Core library API documentation
- [Tauri Architecture](docs/TAURI_ARCHITECTURE.md): Desktop/mobile application architecture
- [Frontend Development](docs/FRONTEND_DEVELOPMENT.md): Guide for Yew frontend development
- [Building and Packaging](docs/BUILDING_AND_PACKAGING.md): Instructions for building and distributing
- [Security Policy](docs/SECURITY.md): Security practices and vulnerability reporting

## Testing

QuietDrop includes comprehensive test suites for all components:

```bash
# Run all tests
cargo test --workspace

# Run specific component tests
cargo test -p quietdrop-core

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

- [Tauri](https://v2.tauri.app/) for the cross-platform application framework
- [Yew](https://yew.rs/) for the frontend framework
- [sodiumoxide](https://github.com/sodiumoxide/sodiumoxide) for cryptographic operations
- [rustls](https://github.com/rustls/rustls) for TLS implementation
- [Tokio](https://github.com/tokio-rs/tokio) for asynchronous runtime
- [Serde](https://github.com/serde-rs/serde) for serialization
- [Argon2](https://github.com/P-H-C/phc-winner-argon2) for password hashing