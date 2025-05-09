# Security Policy

## Reporting a Vulnerability

The QuietDrop team takes security bugs seriously. We appreciate your efforts to responsibly disclose your findings and will make every effort to acknowledge your contributions.

### How to Report a Security Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to [chizy7@outlook.com](mailto:chizy7@outlook.com) or through a private message to the project maintainers.

Please include the following information in your report:

- Type of issue (e.g., buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit the issue

This information will help us triage your report more quickly.

## Disclosure Policy

When we receive a security bug report, we will:

1. Confirm the problem and determine the affected versions
2. Audit code to find any potential similar problems
3. Prepare fixes for all supported releases
4. Release security fixes as soon as possible

## Comments on This Policy

If you have suggestions on how this process could be improved, please submit a pull request or open an issue to discuss.

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Security Update Process

Security updates will be released as part of normal version releases unless an urgent fix is required. In that case, a security patch release will be issued.

## Code Integrity Measures

To ensure the integrity and authenticity of our codebase, we've implemented several security measures:

### Signed Commits

All commits to the QuietDrop repository must be cryptographically signed. This ensures:

1. **Authenticity verification**: Confirms that commits actually come from the claimed author
2. **Protection against account compromise**: Prevents attackers from pushing malicious code if they gain access to a contributor's GitHub account
3. **Chain of custody**: Establishes a verifiable chain of code changes
4. **Tampering detection**: Makes it easier to detect if commits have been modified after they were created

Contributors must set up GPG or SSH commit signing before their contributions can be accepted. For setup instructions, please see:
- Our [CONTRIBUTING.md](../CONTRIBUTING.md#signed-commits) documentation
- [GitHub's official guide on signing commits](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits)

### Branch Protection

We enforce several branch protection rules on critical branches:
- Require signed commits
- Require pull request reviews before merging
- Require status checks to pass before merging
- Require conversation resolution before merging

These measures help prevent unauthorized or unreviewed code from entering the codebase.

## Desktop Application Security

With the migration to Tauri, additional security considerations apply to the desktop application:

### Tauri Security Features

1. **Process Isolation**: The frontend (WebView) and backend (Rust) run in separate processes, providing an additional security boundary
2. **Restricted File System Access**: The WebView has no direct access to the file system; all access is mediated through Tauri APIs
3. **CSP Enforcement**: Content Security Policy is enforced to prevent XSS and other web-based attacks
4. **Custom Protocol**: A custom protocol is used to communicate between the frontend and backend
5. **Permission System**: Tauri provides a permission system for controlling access to system resources

### Additional Measures for Desktop Security

1. **Configuration Hardening**: The default Tauri configuration has been hardened to limit attack surface
2. **Update Security**: The application update process is cryptographically verified
3. **Local Storage Encryption**: Sensitive data stored locally is encrypted using platform-specific mechanisms
4. **Memory Protection**: Sensitive data in memory is properly handled to minimize exposure
5. **Keychain Integration**: Where available, platform keychains are used for storing secrets

## Security-Related Configuration

QuietDrop has been designed with security as a priority. Here are some recommendations for secure deployment:

1. Always keep your QuietDrop installation updated to the latest version
2. Use strong, unique passwords for authentication
3. For server deployments, ensure proper firewall rules are in place
4. For desktop application:
   - Verify the integrity of downloaded installer packages
   - Keep your operating system and security software up to date
   - Enable disk encryption on your device

## Known Security Gaps & Future Enhancements

We are transparent about the current security limitations of QuietDrop:

- The current key exchange mechanism is simplified and will be enhanced in future releases
- Message metadata (sender, timestamp) is currently not encrypted
- Tauri's permission system could be further strengthened for more granular control
- The desktop application currently lacks automatic security updates
- File attachments are not yet implemented with proper content verification

Our security roadmap includes:

- Implementing the Double Ratchet Algorithm for improved forward secrecy
- Adding support for out-of-band key verification
- Encrypting message metadata for enhanced privacy
- Implementing automatic security updates for the desktop application
- Adding secure file attachment handling with content verification
- Regular security audits of the codebase
- Implementing certificate pinning for communication with update servers

## Tauri-Specific Security Recommendations

When using the Tauri desktop application:

1. **Custom Protocol Handling**: Be cautious of custom URL handlers; only interact with `quietdrop://` URLs from trusted sources
2. **External Links**: External links will open in your default browser, not within the application
3. **Updates**: Only download updates through the application's built-in update mechanism
4. **Development Mode**: Do not use development builds in production environments

## Security Expectations for Dependencies

QuietDrop relies on several third-party libraries and frameworks. We make an effort to:

1. Only include dependencies that are actively maintained
2. Regularly update dependencies to incorporate security fixes
3. Evaluate dependencies for security concerns before inclusion
4. Monitor security advisories for all dependencies

## Contact

If you have any questions about this security policy, please contact [chizy7@outlook.com](mailto:chizy7@outlook.com).
