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

## Security-Related Configuration

QuietDrop has been designed with security as a priority. Here are some recommendations for secure deployment:

1. Always keep your QuietDrop installation updated to the latest version
2. Use strong, unique passwords for authentication
3. Ensure server deployments are properly secured with appropriate firewall rules

## Known Security Gaps & Future Enhancements

We are transparent about the current security limitations of QuietDrop:

- The current key exchange mechanism is simplified and will be enhanced in future releases
- Message metadata (sender, timestamp) is currently not encrypted
- (Add any other known limitations here)

Our security roadmap includes:
- Implementing the Double Ratchet Algorithm for improved forward secrecy
- Adding support for out-of-band key verification
- Encrypting message metadata for enhanced privacy
- Regular security audits of the codebase

## Security Expectations for Dependencies

QuietDrop relies on several third-party libraries. We make an effort to:

1. Only include dependencies that are actively maintained
2. Regularly update dependencies to incorporate security fixes
3. Evaluate dependencies for security concerns before inclusion

## Contact

If you have any questions about this security policy, please contact [chizy7@outlook.com](mailto:chizy7@outlook.com).