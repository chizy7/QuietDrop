# Contributing to QuietDrop

Thank you for your interest in contributing to QuietDrop! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Signed Commits](#signed-commits)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Issue Reporting](#issue-reporting)
- [Security Issues](#security-issues)

## Code of Conduct

By participating in this project, you are expected to uphold our [Code of Conduct](CODE_OF_CONDUCT.md). Please read it to understand the expectations for all interactions within the project.

## Getting Started

### Setting Up Your Development Environment

1. **Install Rust and Cargo**
   - Follow the [official Rust installation guide](https://www.rust-lang.org/tools/install)
   - Ensure you have a recent version: `rustc --version` (1.54.0 or newer recommended)

2. **Install Development Tools**
   ```bash
   rustup component add rustfmt clippy
   ```

3. **Fork and Clone the Repository**
   ```bash
   git clone https://github.com/chizy7/QuietDrop.git
   cd QuietDrop
   ```

4. **Set Up Remote**
   ```bash
   git remote add upstream https://github.com/chizy7/QuietDrop.git
   ```

5. **Install Dependencies**
   ```bash
   cargo build
   ```

### Finding Tasks to Work On

- Browse the [GitHub Issues](https://github.com/chizy7/QuietDrop/issues) for tasks labeled with `good first issue` or `help wanted`
- Check the [Project Board](https://github.com/chizy7/QuietDrop/projects) for planned work
- Feel free to ask questions on issues you're interested in

## Development Workflow

1. **Create a Branch**
   ```bash
   git checkout -b feature/my-feature-name
   # or
   git checkout -b fix/issue-description
   ```

   Use the following prefixes for your branches:
   - `feature/` for new features
   - `fix/` for bug fixes
   - `docs/` for documentation changes
   - `refactor/` for code refactoring
   - `test/` for adding or modifying tests

2. **Make Your Changes**
   - Write code following our [coding standards](#coding-standards)
   - Add tests for new functionality
   - Update documentation as needed

3. **Commit Your Changes**
   ```bash
   git add .
   git commit -S -m "Brief description of changes"
   ```

   Write clear, concise commit messages that explain the "what" and "why" of your changes.
   Note the `-S` flag that ensures your commit is signed.

4. **Keep Your Branch Updated**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

5. **Push Your Changes**
   ```bash
   git push origin feature/my-feature-name
   ```

6. **Create a Pull Request**
   - Go to GitHub and create a pull request from your branch
   - Fill out the PR template with details about your changes

## Signed Commits

QuietDrop requires all commits to be cryptographically signed to verify authorship. This helps ensure the authenticity of contributions and is an important security measure for a cryptographic application like ours.

For detailed instructions on setting up and troubleshooting commit signing, please refer to the [GitHub documentation on signing commits](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits).

### Setting up GPG signing

1. **Generate a GPG key** (if you don't already have one):
   ```bash
   gpg --full-generate-key
   ```

2. **List your GPG keys** to get the key ID:
   ```bash
   gpg --list-secret-keys --keyid-format=long
   ```
   Look for a line like `sec   rsa4096/ABCDEF1234567890`

3. **Configure Git to use your GPG key**:
   ```bash
   git config --global user.signingkey ABCDEF1234567890
   git config --global commit.gpgsign true
   ```

4. **Add your GPG key to GitHub**:
   - Export your GPG public key: `gpg --armor --export ABCDEF1234567890`
   - Go to GitHub → Settings → SSH and GPG keys → New GPG key
   - Paste your key and click "Add GPG key"

### Setting up SSH signing (alternative)

GitHub also supports SSH key-based commit verification:

1. **Use an existing SSH key** or generate a new one:
   ```bash
   ssh-keygen -t ed25519 -C "your_email@example.com"
   ```

2. **Configure Git to use SSH signing**:
   ```bash
   git config --global gpg.format ssh
   git config --global user.signingkey ~/.ssh/id_ed25519.pub
   git config --global commit.gpgsign true
   ```

3. **Add your SSH key to GitHub** if you haven't already:
   - Go to GitHub → Settings → SSH and GPG keys → New SSH key
   - Add your public key

### Signing commits

With the above configuration, your commits will be signed automatically.
If you need to sign commits manually:
```bash
git commit -S -m "Your commit message"
```

### Troubleshooting

- **"error: gpg failed to sign the data"**:
  - Check if `echo "test" | gpg --clearsign` works
  - Make sure the email in your GPG key matches your Git email: `git config --global user.email`
  - On macOS, try: `export GPG_TTY=$(tty)`

## Pull Request Process

1. **PR Requirements**
   - All commits must be signed
   - All tests must pass
   - Code must be formatted with `cargo fmt`
   - No new warnings from `cargo clippy`
   - Documentation updated as needed

2. **Code Review**
   - At least one maintainer must review and approve your changes
   - Address all review comments and requested changes
   - Engage in discussion about technical details when needed

3. **Merging**
   - Once approved, a maintainer will merge your PR
   - In some cases, you may be asked to rebase and fix conflicts before merging

## Coding Standards

### Rust Style Guidelines

- Follow the [Rust style guide](https://doc.rust-lang.org/1.0.0/style/README.html)
- Use `cargo fmt` to format your code before committing
- Run `cargo clippy` to check for common mistakes and non-idiomatic code

### General Principles

- **Readability**: Write clear, self-explanatory code
- **Maintainability**: Keep functions small and focused
- **Documentation**: Document public APIs with doc comments
- **Error Handling**: Use proper error handling with `Result` and `?` operator
- **Safety**: Minimize unsafe code and document when it's necessary

### Security Best Practices

- Never hardcode sensitive information
- Use well-established cryptographic libraries
- Follow the principle of least privilege
- Validate all user inputs
- Consider side-channel attacks when implementing cryptographic functionality

## Testing Guidelines

### Types of Tests

- **Unit Tests**: Test individual functions and methods in isolation
- **Integration Tests**: Test component interactions
- **End-to-End Tests**: Test the full application flow

### Writing Tests

- Place unit tests in the same file as the code they test
- Place integration tests in the `tests/` directory
- Name test functions clearly: `test_feature_expected_behavior`
- Use descriptive assertions that explain what's being verified

### Running Tests

```bash
# Run all tests
cargo test

# Run a specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

## Documentation

### Code Documentation

- Document all public API items with doc comments
- Include examples where appropriate
- Explain the purpose, parameters, and return values
- Document panics, errors, and safety concerns

### Project Documentation

- Update README.md when adding major features
- Keep installation and usage instructions current
- Document new configuration options
- Create or update relevant documentation files

## Issue Reporting

### Bug Reports

When reporting bugs, include:
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment details (OS, Rust version, etc.)
- Logs or error messages

### Feature Requests

When requesting features, include:
- Clear description of the desired functionality
- Use cases and benefits
- Any implementation ideas

## Security Issues

For security issues:
- Do not report security vulnerabilities through public GitHub issues
- Email the maintainers directly or use GitHub's security advisories feature
- Include detailed information and potential impact
- Give us reasonable time to address the issue before public disclosure

## License

By contributing to QuietDrop, you agree that your contributions will be licensed under the project's [MIT License](LICENSE).

## Questions and Help

If you have questions or need help:
- Open a discussion in the [GitHub Discussions](https://github.com/chizy7/QuietDrop/discussions)
- Comment on relevant issues
- Reach out to the maintainers

Thank you for contributing to QuietDrop!
