pub mod authentication;
pub mod client;
pub mod encryption;
pub mod message;
pub mod server;

pub fn initialize() {
    // Initialize sodiumoxide if needed
    sodiumoxide::init().expect("Failed to initialize sodiumoxide");
}
