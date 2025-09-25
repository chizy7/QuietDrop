# QuietDrop Project Roadmap

This document outlines the development roadmap for QuietDrop, organized by timeline and priority.

## Completed

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
   - Issue #9: "Create persistent user accounts database"
   - Issue #36: "Implement secure session management"

- [ ] **SQLite Database Integration with sqlx**
   - [ ] Design database schema for users and messages
   - [ ] Implement SQLite storage layer using sqlx
   - [ ] Add migration system for schema changes
   - [ ] Create repository pattern for data access
   - Issue #27: "Implement SQLite database with sqlx"
   - Issue #28: "Create database migration system"

- [ ] **Improved Message Handling**
   - [ ] Add support for message persistence in database
   - [ ] Implement message delivery status (sent, delivered, read)
   - [ ] Support for offline messaging
   - Issue #10: "Implement message storage and history"
   - Issue #37: "Add message status indicators"

### Desktop Application Improvements
- [ ] **Enhanced Tauri Desktop UI**
   - [ ] Implement full conversation view
   - [ ] Add contact management
   - [ ] Improve application styling and UX
   - [ ] Add dark/light theme support
   - Issue #29: "Enhance Tauri desktop UI components"
   - Issue #30: "Implement theming system"

- [ ] **Desktop Application Features**
   - [ ] System tray integration
   - [ ] Desktop notifications
   - [ ] Automatic updates
   - [ ] Message search functionality
   - Issue #31: "Add system tray and notifications"
   - Issue #32: "Implement desktop app update system"

### Infrastructure
- [ ] **Proper Error Handling**
   - [ ] Implement consistent error handling throughout the codebase
   - [ ] Add logging system
   - [ ] Create user-friendly error messages
   - Issue #13: "Implement comprehensive error handling"
   - Issue #45: "Add structured logging system"

- [ ] **Tests**
   - [ ] Unit tests for core functionality
   - [ ] Integration tests for client-server communication
   - [ ] UI tests for Tauri application
   - Issue #: "Add test suite for cryptographic functions" (TODO: update when issue is created)
   - Issue #: "Implement integration tests for messaging" (TODO: update when issue is created)

- [ ] **CI/CD Pipeline**
   - [ ] Set up GitHub Actions for testing and building
   - [ ] Implement automated releases for desktop application
   - [ ] Cross-platform build workflow
   - Issue #46: "Set up CI/CD pipeline with GitHub Actions"

## Short-Term Goals (3-6 Months)

### Features
- [ ] **Group Chat Implementation**
   - [ ] Implement group creation and management
   - [ ] Add group encryption key management
   - [ ] Support for user roles in groups
   - Issue #: "Implement basic group chat functionality" (TODO: update when issue is created)
   - Issue #11: "Add group encryption protocols"

- [ ] **File Transfer**
   - [ ] Support for encrypted file sharing
   - [ ] Progress indicators and resume functionality
   - [ ] File previews for common formats
   - [ ] Automatic media compression
   - Issue #49: "Implement encrypted file transfer"
   - Issue #: "Add support for transfer resume functionality" (TODO: update when issue is created)

- [ ] **Enhanced CLI Interface**
   - [ ] Create a more interactive CLI with menu options
   - [ ] Add command history and navigation
   - [ ] Implement proper terminal UI (TUI) with a library like tui-rs
   - Issue #: "Improve CLI interface with TUI library" (TODO: update when issue is created)

### Infrastructure
- [ ] **Server Architecture Improvements**
   - [ ] Implement proper database backend for server
   - [ ] Add load balancing capabilities
   - [ ] Improve server scalability
   - [ ] Containerization with Docker
   - Issue #: "Integrate database for message and user storage" (TODO: update when issue is created)
   - Issue #: "Implement server load balancing" (TODO: update when issue is created)

- [ ] **Security Auditing**
   - [ ] Conduct security review of cryptographic implementations
   - [ ] Add protections against common attacks
   - [ ] Implement certificate pinning
   - Issue #: "Perform security audit of E2E encryption" (TODO: update when issue is created)
   - Issue #47: "Implement rate limiting and DoS protection"

## Long-Term Goals (6+ Months)

### Features
- [ ] **Mobile Application with Tauri 2.0**
   - [ ] Configure Tauri 2.0 for mobile targets
   - [ ] Adapt UI for mobile screen sizes
   - [ ] Implement mobile-specific features (contacts sync, etc.)
   - [ ] Add push notification support
   - Issue #42: "Configure Tauri 2.0 for mobile targets"
   - Issue #43: "Implement mobile-specific UI adaptations"

- [ ] **Advanced Features**
   - [ ] Self-destructing messages
   - [ ] Voice and video calls
   - [ ] End-to-end encrypted backups
   - [ ] Message scheduling
   - Issue #51: "Implement ephemeral messaging"
   - Issue #: "Add voice/video calling capability" (TODO: update when issue is created)

- [ ] **Offline Capabilities**
   - [ ] Implement robust offline message queue
   - [ ] Add local-first data architecture
   - [ ] Synchronization protocols for multiple devices
   - Issue #38: "Implement offline message queue"
   - Issue #52: "Create multi-device synchronization"

### Infrastructure
- [ ] **Federation Protocol**
   - [ ] Design and implement server federation
   - [ ] Support for decentralized communication
   - [ ] Compatibility with existing messaging protocols
   - Issue #: "Design federation protocol specification" (TODO: update when issue is created)
   - Issue #: "Implement basic server federation" (TODO: update when issue is created)

- [ ] **Extensibility**
   - [ ] Plugin system for additional features
   - [ ] API for third-party integrations
   - [ ] SDK for developers
   - Issue #67: "Design plugin architecture"
   - Issue #: "Create public API for integrations" (TODO: update when issue is created)

## Technical Debt & Maintenance

- [ ] **Code Refactoring**
   - [ ] Improve code organization
   - [ ] Enhance documentation
   - [ ] Standardize coding patterns
   - Issue #: "Refactor client-server communication layer" (TODO: update when issue is created)

- [ ] **Dependency Management**
   - [ ] Regular updates of dependencies
   - [ ] Security patch management
   - [ ] Dependency audit workflow
   - Issue #: "Set up dependency update automation" (TODO: update when issue is created)

- [ ] **Performance Optimization**
   - [ ] Profile and optimize cryptographic operations
   - [ ] Reduce memory usage
   - [ ] Improve startup time for desktop/mobile applications
   - [ ] Minimize bundle size
   - Issue #: "Optimize message encryption/decryption performance" (TODO: update when issue is created)

## Review and Planning

This roadmap will be reviewed and updated quarterly. As the project evolves, priorities may shift based on user feedback, technological changes, and resource availability.

If you're interested in contributing to any of these initiatives, please check the associated GitHub issues or reach out to the maintainers.
