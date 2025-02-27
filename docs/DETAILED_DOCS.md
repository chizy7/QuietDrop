# QuietDrop Documentation

## Overview

QuietDrop is an end-to-end encrypted chat application built in Rust. It prioritizes security, privacy, and performance while providing a simple communication platform.

## Architecture

### Current Implementation

QuietDrop follows a client-server architecture:

```
┌─────────┐                ┌─────────┐
│         │◄──Encrypted──►│         │
│ Client  │    Messages    │ Server  │
│         │                │         │
└─────────┘                └─────────┘
```

The system consists of the following components:

1. **Authentication Module** (`authentication.rs`)
   - Handles password hashing and verification using Argon2
   - Manages salt generation and storage

2. **Encryption Module** (`encryption.rs`)
   - Uses libsodium (via sodiumoxide) for cryptographic operations
   - Implements public-key encryption with the NaCl crypto_box
   - Handles key pair generation and message encryption/decryption

3. **Client Module** (`client.rs`)
   - Manages connection to the server
   - Handles sending encrypted messages

4. **Server Module** (`server.rs`)
   - Listens for incoming connections
   - Processes and decrypts received messages
   - Includes basic rate limiting functionality

5. **Message Module** (`message.rs`)
   - Defines message structure and types
   - Provides utility functions for message handling

### Data Flow

1. **Key Exchange**:
   - Both server and client generate their key pairs (public and secret keys)
   - The server's public key is stored and shared with clients
   - Each client generates its own key pair and sends its public key with each message

2. **Message Encryption**:
   - Messages are encrypted on the client side using the server's public key and client's secret key
   - The encrypted message is sent along with the client's public key

3. **Message Decryption**:
   - The server decrypts messages using the client's public key and its own secret key

## Security Features

### End-to-End Encryption

QuietDrop uses the NaCl cryptographic library (via sodiumoxide) to implement public-key authenticated encryption:

- **Algorithm**: X25519-XSalsa20-Poly1305 (crypto_box)
- **Key Exchange**: Currently manual exchange of public keys
- **Authentication**: Poly1305 message authentication code ensures message integrity

### Password Security

User passwords are secured using:

- **Hashing Algorithm**: Argon2id (memory-hard function)
- **Parameterization**: Configurable memory, iterations, and parallelism parameters
- **Salt**: Randomly generated and stored separately

## Core Components

### Authentication System

```rust
// Hash a password with Argon2id
pub fn hash_password(password: &str) -> Result<(String, String)>

// Verify a password against stored hash
pub fn verify_password(hashed_password: &str, salt: &str, password: &str) -> Result<bool>
```

### Encryption System

```rust
// Generate a new keypair
pub fn generate_keypair() -> KeyPair

// Encrypt a message using receiver's public key and sender's secret key
pub fn encrypt_message(message: &str, public_key: &PublicKey, secret_key: &SecretKey) -> Vec<u8>

// Decrypt a message using sender's public key and receiver's secret key
pub fn decrypt_message(encrypted_data: &[u8], public_key: &PublicKey, secret_key: &SecretKey) -> Result<String, &'static str>
```

### Message Format

Messages are structured as follows:

```rust
pub struct Message {
    pub timestamp: DateTime<Utc>,     // When the message was sent
    pub message_type: MessageType,    // Type of message (Text or File)
    pub sender: String,               // Sender's identifier
    pub recipient: String,            // Recipient's identifier
    pub content: Vec<u8>,             // Encrypted message content
    pub public_key: PublicKey,        // Sender's public key
}
```

## Setup and Usage

### Prerequisites

- Rust 1.54.0 or newer
- Cargo package manager

### Building From Source

1. Clone the repository:
   ```bash
   git clone https://github.com/chizy7/QuietDrop.git
   cd QuietDrop
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

### Running the Server

Run the server component:

```bash
cargo run server
```

This will:
- Generate a new server keypair if one doesn't exist
- Save the public and secret keys to `server_public_key.key` and `server_secret_key.key`
- Start listening for connections on `127.0.0.1:8080`

### Running the Client

In a separate terminal, run the client:

```bash
cargo run client
```

This will:
- Generate a client keypair
- Read the server's public key
- Prompt for a name and message
- Encrypt and send the message to the server

## Development Workflow

### Code Organization

- **Source Files**: All Rust source files are in the `src` directory
- **Documentation**: Located in the `documentation` directory
- **Build Artifacts**: Generated in the `target` directory (not tracked in git)

### Project Structure

```
QuietDrop/
├── Cargo.lock          - Lock file for dependencies
├── Cargo.toml          - Project configuration
├── src/                - Source code
│   ├── authentication.rs  - Authentication functionality
│   ├── client.rs       - Client implementation
│   ├── encryption.rs   - Encryption operations
│   ├── lib.rs          - Library exports
│   ├── main.rs         - Application entry point
│   ├── message.rs      - Message handling
│   └── server.rs       - Server implementation
├── documentation/      - Project documentation
└── README.md           - Project overview
```

### Coding Standards

- Follow Rust style conventions using `rustfmt`
- Run `cargo fmt` before committing changes
- Use `cargo clippy` for linting
- Write meaningful commit messages

## Contributing

We welcome contributions! Here's how to get started:

1. **Pick an Issue**: Check the GitHub issues for tasks to work on
2. **Fork the Repository**: Create your own fork of the project
3. **Create a Branch**: Make your changes in a new branch
4. **Make Changes**: Implement your feature or fix
5. **Run Tests**: Ensure all tests pass with `cargo test`
6. **Format Code**: Run `cargo fmt` to format your code
7. **Submit a PR**: Open a pull request with your changes

### Development Environment Setup

1. Install the Rust toolchain:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install development tools:
   ```bash
   rustup component add rustfmt clippy
   ```

3. Set up Git hooks (recommended):
   ```bash
   # Create a pre-commit hook for formatting
   echo '#!/bin/sh
   cargo fmt -- --check
   cargo clippy -- -D warnings' > .git/hooks/pre-commit
   chmod +x .git/hooks/pre-commit
   ```

## Future Plans

See the [Project Roadmap](PROJECT_ROADMAP.md) for detailed information about:
- Immediate development goals
- Short-term features
- Long-term roadmap
- Technical debt items

## License

QuietDrop is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.