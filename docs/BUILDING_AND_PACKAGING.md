# QuietDrop Building and Packaging Guide

This document provides comprehensive instructions for building, packaging, and distributing the QuietDrop application for different platforms using Tauri 2.0.

## Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Building the CLI](#building-the-cli)
- [Building the Cross-Platform Application](#building-the-cross-platform-application)
- [Packaging for Distribution](#packaging-for-distribution)
- [Continuous Integration](#continuous-integration)
- [Release Process](#release-process)
- [Update Mechanism](#update-mechanism)
- [Troubleshooting](#troubleshooting)

## Overview

QuietDrop consists of multiple components that need to be built:

1. **Core Library** (`quietdrop-core`): The shared functionality
2. **CLI Application** (`quietdrop-cli`): Command-line interface
3. **Cross-Platform Application** (`quietdrop-tauri`): Tauri 2.0-based application for desktop and mobile

The build process involves compiling Rust code, compiling WebAssembly for the frontend, and packaging everything into installable applications for various platforms.

## Prerequisites

### System Requirements

- **Rust** (1.70.0 or newer)
- **Cargo** (comes with Rust)
- **Node.js** (for Tauri's build system, not for application code)
- **Platform-specific dependencies** (detailed below)

### Platform-specific Dependencies

#### Windows

- Visual Studio C++ Build Tools
- WebView2
- Windows 10 SDK

```bash
# Install with Chocolatey
choco install visualstudio2019-workload-vctools webview2-runtime windows-sdk-10.0
```

#### macOS

- Xcode Command Line Tools
- macOS 10.15 or newer

```bash
# Install with Homebrew
brew install gcc
xcode-select --install
```

#### Linux

- Various development libraries

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# Fedora
sudo dnf install webkit2gtk3-devel openssl-devel curl wget file libappindicator-gtk3-devel librsvg2-devel

# Arch
sudo pacman -S webkit2gtk base-devel curl wget file openssl gtk3 libappindicator-gtk3 librsvg
```

#### Android

- Android SDK
- Android NDK
- Java Development Kit (JDK 11+)

```bash
# Install Android Studio (recommended) or Android command-line tools
# Then install required SDK components:
sdkmanager "platform-tools" "platforms;android-33" "build-tools;33.0.0" "ndk;25.1.8937393"
```

#### iOS

- Xcode (latest version)
- iOS development certificates
- Cocoapods

```bash
# Install Cocoapods
sudo gem install cocoapods
```

### Build Tools

Install Rust and related tools:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Trunk (for building Yew)
cargo install trunk

# Install Tauri CLI
cargo install tauri-cli
```

## Building the CLI

The command-line interface can be built separately from the cross-platform application:

```bash
# Navigate to the project root
cd QuietDrop

# Build the CLI in debug mode
cargo build -p quietdrop-cli

# Build in release mode for better performance
cargo build -p quietdrop-cli --release

# The binary will be in target/release/quietdrop-cli
```

### Running the CLI

```bash
# Run in server mode
./target/release/quietdrop-cli server

# Run in client mode
./target/release/quietdrop-cli client
```

## Building the Cross-Platform Application

Tauri 2.0 supports building for desktop and mobile platforms from the same codebase:

### Desktop Builds

```bash
# Navigate to the Tauri project
cd quietdrop-tauri

# Development build with hot reloading
cargo tauri dev

# Production build for the current platform
cargo tauri build
```

### Android Builds

```bash
# Ensure Android SDK and NDK are properly set up
# You may need to configure ANDROID_HOME and ANDROID_NDK_HOME environment variables

# Development build for Android
cd quietdrop-tauri
cargo tauri android dev

# Production build for Android
cargo tauri android build --release
```

### iOS Builds

```bash
# Ensure Xcode and iOS certificates are properly set up

# Development build for iOS
cd quietdrop-tauri
cargo tauri ios dev

# Production build for iOS
cargo tauri ios build --release
```

## Packaging for Distribution

Tauri 2.0 automatically creates distribution packages for the target platforms.

### Output Locations

After running the build commands, you'll find the packaged applications in:

**Desktop:**
```
quietdrop-tauri/target/release/bundle/
```

**Android:**
```
quietdrop-tauri/gen/android/app/build/outputs/apk/release/app-release.apk
quietdrop-tauri/gen/android/app/build/outputs/bundle/release/app-release.aab
```

**iOS:**
```
quietdrop-tauri/gen/ios/App/build/Products/Release-iphoneos/App.app
```

The output includes:

- **Windows**: `.msi` installer, `.exe` setup
- **macOS**: `.dmg` disk image, `.app` bundle
- **Linux**: `.deb`, `.AppImage`, other formats based on configuration
- **Android**: `.apk` application package, `.aab` Android App Bundle
- **iOS**: `.app` application bundle, `.ipa` iOS App Store package

### Customizing the Packaging

Customize packaging in `src-tauri/tauri.conf.json`. Below is an example based on your provided configuration:

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "QuietDrop",
  "version": "0.1.0",
  "identifier": "com.quietdrop.app",
  "build": {
    "frontendDist": "../dist",
    "devUrl": "http://localhost:1420",
    "beforeDevCommand": "",
    "beforeBuildCommand": "",
    "withGlobalTauri": true
  },
  "app": {
    "windows": [
      {
        "title": "QuietDrop",
        "width": 800,
        "height": 600,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": "default-src 'self'; style-src 'self' 'unsafe-inline'"
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "category": "Utility",
    "copyright": "",
    "deb": {
      "depends": []
    },
    "externalBin": [],
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "longDescription": "End-to-End Encrypted Messaging Application",
    "macOS": {
      "entitlements": null,
      "exceptionDomain": "",
      "frameworks": [],
      "providerShortName": null,
      "signingIdentity": null
    },
    "resources": [],
    "shortDescription": "Secure Messaging App",
    "windows": {
      "certificateThumbprint": null,
      "digestAlgorithm": "sha256",
      "timestampUrl": ""
    }
  },
  "updater": {
    "active": true,
    "endpoints": [
      "https://releases.quietdrop.com/{{target}}/{{current_version}}"
    ],
    "dialog": true,
    "pubkey": ""
  },
  "android": {
    "package": "com.quietdrop.app",
    "versionCode": 1,
    "minSdkVersion": 24,
    "targetSdkVersion": 33
  },
  "ios": {
    "developmentTeam": "TEAM_ID",
    "minimumOSVersion": "13.0"
  }
}
```

### Android Configuration

Additional Android configurations can be added to the Android project after initial setup.

### iOS Configuration

Additional iOS configurations can be set in the Xcode project after initial setup.

## Continuous Integration

QuietDrop uses GitHub Actions for CI/CD:

### Sample GitHub Actions Workflow for Cross-Platform Builds

```yaml
name: Build and Release

on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:

jobs:
  build-desktop:
    strategy:
      fail-fast: false
      matrix:
        platform: [macos-latest, ubuntu-latest, windows-latest]

    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      
      - name: Install dependencies (ubuntu)
        if: matrix.platform == 'ubuntu-latest'
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.0-dev build-essential libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
      
      - name: Install Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 'lts/*'
      
      - name: Build desktop app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          projectPath: ./quietdrop-tauri
          
  build-android:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Java
        uses: actions/setup-java@v3
        with:
          distribution: 'temurin'
          java-version: '17'
      
      - name: Setup Android SDK
        uses: android-actions/setup-android@v2
      
      - name: Build Android app
        run: |
          cd quietdrop-tauri
          cargo tauri android build --release
      
      - name: Upload APK
        uses: actions/upload-artifact@v3
        with:
          name: android-release
          path: quietdrop-tauri/gen/android/app/build/outputs/apk/release/app-release.apk
          
  build-ios:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Xcode
        uses: maxim-lobanov/setup-xcode@v1
        with:
          xcode-version: latest-stable
      
      - name: Install Cocoapods
        run: sudo gem install cocoapods
      
      - name: Build iOS app
        run: |
          cd quietdrop-tauri
          cargo tauri ios build
        env:
          APPLE_TEAM_ID: ${{ secrets.APPLE_TEAM_ID }}
      
      - name: Upload iOS package
        uses: actions/upload-artifact@v3
        with:
          name: ios-release
          path: quietdrop-tauri/gen/ios/App/build/Products/Release-iphoneos/App.app
```

## Release Process

### Version Management

Update version in `tauri.conf.json`:
```json
{
  "version": "0.1.0"
}
```

### Creating a Release

1. Create and push a git tag:
   ```bash
   git tag -a v0.1.0 -m "Release v0.1.0"
   git push origin v0.1.0
   ```

2. The CI pipeline will build the release artifacts.

3. Add release notes on GitHub.

## Update Mechanism

### Desktop Update Mechanism

QuietDrop can use Tauri's built-in updater for automatic updates for desktop applications.

#### Setting Up the Update Server

1. Host the update artifacts on a server.
2. Create an update manifest file.
3. Generate signatures for each package.

### Mobile Update Mechanisms

Mobile applications typically use platform-specific update mechanisms:

#### Android Updates

- **Google Play Store**: Standard updates through the Play Store
- **In-App Updates**: Implement the Play Core Library for in-app updates

#### iOS Updates

- **App Store**: Standard updates through the App Store
- **TestFlight**: For beta testing

## Troubleshooting

### Common Build Issues

#### WebAssembly Compilation Errors

```
Error: Failed to execute `cargo build`
```

**Solution**: Ensure the WebAssembly target is installed: `rustup target add wasm32-unknown-unknown`

#### Missing Dependencies (Linux)

```
Error: Failed to load shared library: libwebkit2gtk-4.0.so
```

**Solution**: Install the required system packages as described in the [Prerequisites](#prerequisites) section.

#### Tauri Configuration Errors

```
Error: Invalid configuration: Could not find index.html
```

**Solution**: Ensure the `frontendDist` in `tauri.conf.json` points to the correct directory containing your built frontend.

#### Android Build Errors

```
Error: Android SDK not found
```

**Solution**: Set the `ANDROID_HOME` environment variable to your Android SDK location.

#### iOS Build Errors

```
Error: Xcode not found
```

**Solution**: Install Xcode from the App Store and run `xcode-select --install`.

### Debugging Tips

1. **Enable verbose output**:
   ```bash
   RUST_LOG=debug cargo tauri build
   ```

2. **Check log files**:
   - Windows: `%APPDATA%\com.quietdrop.app\logs`
   - macOS: `~/Library/Logs/com.quietdrop.app`
   - Linux: `~/.config/com.quietdrop.app/logs`
   - Android: Connect device and use `adb logcat`
   - iOS: Use Xcode's Console app

3. **Inspect the built artifacts**

## Best Practices

1. **Version Control**: Keep all build configuration in version control.
2. **Automated Builds**: Use CI/CD for consistent builds across platforms.
3. **Testing**: Test the packaged application, not just the development build.
4. **Incremental Updates**: Use small, frequent updates rather than infrequent large ones.
5. **Security**: Always sign your releases and verify signatures during updates.
6. **Documentation**: Keep documentation up to date with each release.
7. **Backup**: Securely back up your signing keys and certificates.

## Conclusion

This guide covers the essential steps for building, packaging, and distributing QuietDrop across different platforms using Tauri 2.0. By following these instructions, you can create reliable release builds for desktop and mobile platforms and maintain an effective update workflow.

For further assistance or to report build issues, please submit a GitHub issue or contact the project maintainers.