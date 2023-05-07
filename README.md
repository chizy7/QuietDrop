# SecureChat

SecureChat is an end-to-end encrypted chat application built using Rust. It provides a secure, private, and easy-to-use communication platform for users, leveraging Rust's memory safety and performance benefits. SecureChat uses modern cryptographic libraries and protocols to ensure the confidentiality and integrity of messages exchanged between users.

## Features

* End-to-end encryption: Messages are encrypted on the sender's device and decrypted on the receiver's device, ensuring that even if the messages are intercepted, they cannot be read without the proper decryption keys.
* Authentication: Users can create accounts with strong passwords and authenticate themselves to the system.
* Forward secrecy: The system implements the concept of forward secrecy, ensuring that if a user's private key is compromised, past messages cannot be decrypted.
* Group chats: Users can create and participate in group chats, with all messages encrypted and decrypted among group members.
* File transfer: The application supports encrypted file transfer between users.

## Getting Started

### Prerequisites
* [Rust](https://www.rust-lang.org/tools/install) programming language (1.54.0 or later)
* [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html), the Rust package manager

### Installation
1. Clone the repository:
```
git clone https://github.com/chizy7/SecureChat.git
cd securechat
```
2. Build the project:
```
cargo build
```
### Usage
1. Start the server:
```
cargo run server
```
2. Start the client in another terminal:
```
cargo run client
```

Note that the current implementation provides a simple CLI for testing purposes. You can extend the project to create a more user-friendly client application with a graphical user interface (GUI) or a more sophisticated command-line interface (CLI).

## Contributing

Contributions are welcome! Please read the [contributing guide](CONTRIBUTING) for more information on how to get involved in the project.

## License

This project is licensed under the MIT [License](LICENSE).

## Acknowledgements

* [sodiumoxide](https://github.com/sodiumoxide/sodiumoxide) for cryptographic operations 
* [rustls](https://github.com/rustls/rustls) for TLS implementation 
* [Tokio](https://github.com/tokio-rs/tokio) for asynchronous network programming 
* [Serde](https://github.com/serde-rs/serde) for serialization and deserialization of data 
* [serde_json](https://github.com/serde-rs/json) for encoding and decoding messages in JSON format 