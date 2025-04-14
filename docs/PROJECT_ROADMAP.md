# QuietDrop Project Roadmap (TODO: To be corrected with the right issues number)

This document outlines the development roadmap for QuietDrop, organized by timeline and priority.

## ✅ Completed

- [x] **Initial CLI Implementation**
   - Basic client-server communication
   - End-to-end encryption using sodiumoxide
   - Command-line interface for sending messages

- [x] **Project Refactoring to Workspace Structure**
   - Separated core library from interfaces
   - Created workspace with multiple crates
   - Improved code organization

- [x] **Tauri 2.0 Application Scaffold**
   - Initial Tauri application setup
   - Basic Yew frontend implementation
   - Integration with core library

## Immediate Goals (Next 1-2 Months)

### Core Functionality
- [ ] **User Registration & Authentication System** 
   - [ ] Implement proper user account creation and storage
   - [ ] Add session management
   - [ ] Implement password reset functionality
   - Issue #1: "Create persistent user accounts database"
   - Issue #2: "Implement secure session management"

- [ ] **SQLite Database Integration with sqlx**
   - [ ] Design database schema for users and messages
   - [ ] Implement SQLite storage layer using sqlx
   - [ ] Add migration system for schema changes
   - [ ] Create repository pattern for data access
   - Issue #32: "Implement SQLite database with sqlx"
   - Issue #33: "Create database migration system"

- [ ] **Improved Message Handling**
   - [ ] Add support for message persistence in database
   - [ ] Implement message delivery status (sent, delivered, read)
   - [ ] Support for offline messaging
   - Issue #3: "Implement message storage and history"
   - Issue #4: "Add message status indicators"

### Desktop Application Improvements
- [ ] **Enhanced Tauri Desktop UI**
   - [ ] Implement full conversation view
   - [ ] Add contact management
   - [ ] Improve application styling and UX
   - [ ] Add dark/light theme support
   - Issue #34: "Enhance Tauri desktop UI components"
   - Issue #35: "Implement theming system"

- [ ] **Desktop Application Features**
   - [ ] System tray integration
   - [ ] Desktop notifications
   - [ ] Automatic updates
   - [ ] Message search functionality
   - Issue #36: "Add system tray and notifications"
   - Issue #37: "Implement desktop app update system"

### Infrastructure
- [ ] **Proper Error Handling**
   - [ ] Implement consistent error handling throughout the codebase
   - [ ] Add logging system
   - [ ] Create user-friendly error messages
   - Issue #6: "Implement comprehensive error handling"
   - Issue #7: "Add structured logging system"

- [ ] **Tests**
   - [ ] Unit tests for core functionality
   - [ ] Integration tests for client-server communication
   - [ ] UI tests for Tauri application
   - Issue #8: "Add test suite for cryptographic functions"
   - Issue #9: "Implement integration tests for messaging"

- [ ] **CI/CD Pipeline**
   - [ ] Set up GitHub Actions for testing and building
   - [ ] Implement automated releases for desktop application
   - [ ] Cross-platform build workflow
   - Issue #10: "Set up CI/CD pipeline with GitHub Actions"

## Short-Term Goals (3-6 Months)

### Features
- [ ] **Group Chat Implementation**
   - [ ] Implement group creation and management
   - [ ] Add group encryption key management
   - [ ] Support for user roles in groups
   - Issue #11: "Implement basic group chat functionality"
   - Issue #12: "Add group encryption protocols"

- [ ] **File Transfer**
   - [ ] Support for encrypted file sharing
   - [ ] Progress indicators and resume functionality
   - [ ] File previews for common formats
   - [ ] Automatic media compression
   - Issue #13: "Implement encrypted file transfer"
   - Issue #14: "Add support for transfer resume functionality"

- [ ] **Enhanced CLI Interface**
   - [ ] Create a more interactive CLI with menu options
   - [ ] Add command history and navigation
   - [ ] Implement proper terminal UI (TUI) with a library like tui-rs
   - Issue #5: "Improve CLI interface with TUI library"

### Infrastructure
- [ ] **Server Architecture Improvements**
   - [ ] Implement proper database backend for server
   - [ ] Add load balancing capabilities
   - [ ] Improve server scalability
   - [ ] Containerization with Docker
   - Issue #16: "Integrate database for message and user storage"
   - Issue #17: "Implement server load balancing"

- [ ] **Security Auditing**
   - [ ] Conduct security review of cryptographic implementations
   - [ ] Add protections against common attacks
   - [ ] Implement certificate pinning
   - Issue #18: "Perform security audit of E2E encryption"
   - Issue #19: "Implement rate limiting and DoS protection"

## Long-Term Goals (6+ Months)

### Features
- [ ] **Mobile Application with Tauri 2.0**
   - [ ] Configure Tauri 2.0 for mobile targets
   - [ ] Adapt UI for mobile screen sizes
   - [ ] Implement mobile-specific features (contacts sync, etc.)
   - [ ] Add push notification support
   - Issue #39: "Configure Tauri 2.0 for mobile targets"
   - Issue #40: "Implement mobile-specific UI adaptations"

- [ ] **Advanced Features**
   - [ ] Self-destructing messages
   - [ ] Voice and video calls
   - [ ] End-to-end encrypted backups
   - [ ] Message scheduling
   - Issue #23: "Implement ephemeral messaging"
   - Issue #24: "Add voice/video calling capability"

- [ ] **Offline Capabilities**
   - [ ] Implement robust offline message queue
   - [ ] Add local-first data architecture
   - [ ] Synchronization protocols for multiple devices
   - Issue #41: "Implement offline message queue"
   - Issue #42: "Create multi-device synchronization"

### Infrastructure
- [ ] **Federation Protocol**
   - [ ] Design and implement server federation
   - [ ] Support for decentralized communication
   - [ ] Compatibility with existing messaging protocols
   - Issue #25: "Design federation protocol specification"
   - Issue #26: "Implement basic server federation"

- [ ] **Extensibility**
   - [ ] Plugin system for additional features
   - [ ] API for third-party integrations
   - [ ] SDK for developers
   - Issue #27: "Design plugin architecture"
   - Issue #28: "Create public API for integrations"

## Technical Debt & Maintenance

- [ ] **Code Refactoring**
   - [ ] Improve code organization
   - [ ] Enhance documentation
   - [ ] Standardize coding patterns
   - Issue #29: "Refactor client-server communication layer"
   
- [ ] **Dependency Management**
   - [ ] Regular updates of dependencies
   - [ ] Security patch management
   - [ ] Dependency audit workflow
   - Issue #30: "Set up dependency update automation"

- [ ] **Performance Optimization**
   - [ ] Profile and optimize cryptographic operations
   - [ ] Reduce memory usage
   - [ ] Improve startup time for desktop/mobile applications
   - [ ] Minimize bundle size
   - Issue #31: "Optimize message encryption/decryption performance"

## Review and Planning

This roadmap will be reviewed and updated quarterly. As the project evolves, priorities may shift based on user feedback, technological changes, and resource availability.

If you're interested in contributing to any of these initiatives, please check the associated GitHub issues or reach out to the maintainers.