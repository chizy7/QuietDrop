# QuietDrop Project Roadmap - Draft Version (to be correctly updated later with accurate issues numbers)

## Immediate Goals (Next 1-2 Months)

### Core Functionality
1. **User Registration & Authentication System** 
   - Implement proper user account creation and storage
   - Add session management
   - Implement password reset functionality
   - Issue #1: "Create persistent user accounts database"
   - Issue #2: "Implement secure session management"

2. **Improved Message Handling**
   - Add support for message persistence
   - Implement message delivery status (sent, delivered, read)
   - Support for offline messaging
   - Issue #3: "Implement message storage and history"
   - Issue #4: "Add message status indicators"

3. **Enhanced CLI Interface**
   - Create a more interactive CLI with menu options
   - Add command history and navigation
   - Implement proper terminal UI (TUI) with a library like tui-rs
   - Issue #5: "Improve CLI interface with TUI library"

### Infrastructure
1. **Proper Error Handling**
   - Implement consistent error handling throughout the codebase
   - Add logging system
   - Issue #6: "Implement comprehensive error handling"
   - Issue #7: "Add structured logging system"

2. **Tests**
   - Unit tests for core functionality
   - Integration tests for client-server communication
   - Issue #8: "Add test suite for cryptographic functions"
   - Issue #9: "Implement integration tests for messaging"

3. **CI/CD Pipeline**
   - Set up GitHub Actions for testing and building
   - Implement automated releases
   - Issue #10: "Set up CI/CD pipeline with GitHub Actions"

## Short-Term Goals (3-6 Months)

### Features
1. **Group Chat Implementation**
   - Implement group creation and management
   - Add group encryption key management
   - Support for user roles in groups
   - Issue #11: "Implement basic group chat functionality"
   - Issue #12: "Add group encryption protocols"

2. **File Transfer**
   - Support for encrypted file sharing
   - Progress indicators and resume functionality
   - File previews for common formats
   - Issue #13: "Implement encrypted file transfer"
   - Issue #14: "Add support for transfer resume functionality"

3. **Simple GUI Client**
   - Basic desktop GUI using a Rust GUI framework (e.g., iced, egui)
   - Cross-platform support (Windows, macOS, Linux)
   - Issue #15: "Create basic GUI client with iced/egui"

### Infrastructure
1. **Server Architecture Improvements**
   - Implement proper database backend
   - Add load balancing capabilities
   - Improve server scalability
   - Issue #16: "Integrate database for message and user storage"
   - Issue #17: "Implement server load balancing"

2. **Security Auditing**
   - Conduct security review of cryptographic implementations
   - Add protections against common attacks
   - Issue #18: "Perform security audit of E2E encryption"
   - Issue #19: "Implement rate limiting and DoS protection"

## Long-Term Goals (6+ Months)

### Features
1. **Web Client**
   - Develop web interface using a framework like Yew or Seed
   - WebAssembly integration for cryptographic operations
   - Issue #20: "Create web client with Rust-to-WASM compilation"

2. **Mobile Applications**
   - Develop mobile clients for Android and iOS
   - Implement push notifications
   - Focus on battery-efficient operations
   - Issue #21: "Develop Android client using Rust FFI"
   - Issue #22: "Develop iOS client using Rust FFI"

3. **Advanced Features**
   - Self-destructing messages
   - Voice and video calls
   - End-to-end encrypted backups
   - Issue #23: "Implement ephemeral messaging"
   - Issue #24: "Add voice/video calling capability"

### Infrastructure
1. **Federation Protocol**
   - Design and implement server federation
   - Support for decentralized communication
   - Issue #25: "Design federation protocol specification"
   - Issue #26: "Implement basic server federation"

2. **Extensibility**
   - Plugin system for additional features
   - API for third-party integrations
   - Issue #27: "Design plugin architecture"
   - Issue #28: "Create public API for integrations"

## Technical Debt & Maintenance

1. **Code Refactoring**
   - Improve code organization
   - Enhance documentation
   - Issue #29: "Refactor client-server communication layer"
   
2. **Dependency Management**
   - Regular updates of dependencies
   - Security patch management
   - Issue #30: "Set up dependency update automation"

3. **Performance Optimization**
   - Profile and optimize cryptographic operations
   - Reduce memory usage
   - Issue #31: "Optimize message encryption/decryption performance"