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
- **Node.js** (16.0.0 or newer, for Tauri)
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

Customize packaging in `src-tauri/tauri.conf.json`:

```json
{
  "package": {
    "productName": "QuietDrop",
    "version": "0.1.0"
  },
  "build": {
    "distDir": "../dist",
    "devPath": "http://localhost:1420",
    "beforeDevCommand": "",
    "beforeBuildCommand": "",
    "withGlobalTauri": true
  },
  "tauri": {
    "bundle": {
      "active": true,
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
      "identifier": "com.quietdrop.app",
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
      "targets": ["deb", "appimage", "msi", "app", "dmg", "updater"],
      "windows": {
        "certificateThumbprint": null,
        "digestAlgorithm": "sha256",
        "timestampUrl": ""
      }
    },
    "security": {
      "csp": "default-src 'self'; style-src 'self' 'unsafe-inline'"
    },
    "updater": {
      "active": true,
      "endpoints": [
        "https://releases.quietdrop.com/{{target}}/{{current_version}}"
      ],
      "dialog": true,
      "pubkey": ""
    },
    "windows": [
      {
        "title": "QuietDrop",
        "width": 800,
        "height": 600,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "android": {
      "package": "com.quietdrop.app",
      "versionCode": 1,
      "minSdkVersion": 24,
      "targetSdkVersion": 33
    },
    "ios": {
      "developmentTeam": "YOUR_TEAM_ID",
      "minimumOSVersion": "13.0"
    }
  }
}
```

### Mobile-Specific Configuration

#### Android Configuration

Additional Android configurations can be set in `gen/android/app/build.gradle` after initial setup:

```gradle
android {
    // ...
    
    defaultConfig {
        // ...
        versionCode 1
        versionName "1.0.0"
        
        // Enable multidex support if needed
        multiDexEnabled true
    }
    
    signingConfigs {
        release {
            storeFile file("keystore.jks")
            storePassword System.getenv("KEYSTORE_PASSWORD")
            keyAlias System.getenv("KEY_ALIAS")
            keyPassword System.getenv("KEY_PASSWORD")
        }
    }
    
    buildTypes {
        release {
            minifyEnabled true
            proguardFiles getDefaultProguardFile('proguard-android-optimize.txt'), 'proguard-rules.pro'
            signingConfig signingConfigs.release
        }
    }
}
```

#### iOS Configuration

Additional iOS configurations can be set in `gen/ios/App/App.xcodeproj/project.pbxproj` or through Xcode:

1. Open the generated Xcode project
2. Configure signing certificates
3. Set app capabilities (e.g., Push Notifications, Background Modes)
4. Configure privacy descriptions in `Info.plist`

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
      
      - name: Build desktop app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          projectPath: ./quietdrop-tauri
          tagName: v__VERSION__
          releaseName: "QuietDrop v__VERSION__"
          releaseBody: "See the assets to download the installer for your platform."
          releaseDraft: true
          prerelease: false
          
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

1. Update version in `Cargo.toml` files:
   ```toml
   # quietdrop-core/Cargo.toml
   [package]
   name = "quietdrop-core"
   version = "0.1.0"  # Update this
   ```

2. Update version in `tauri.conf.json`:
   ```json
   {
     "package": {
       "version": "0.1.0"  # Update this
     },
     "tauri": {
       "android": {
         "versionCode": 1  # Increment this for Android
       }
     }
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

QuietDrop uses Tauri's built-in updater for automatic updates for desktop applications:

#### Updater Configuration

Configure in `tauri.conf.json`:

```json
"updater": {
  "active": true,
  "endpoints": [
    "https://releases.quietdrop.com/{{target}}/{{current_version}}"
  ],
  "dialog": true,
  "pubkey": "YOUR_PUBLIC_KEY_HERE"
}
```

#### Setting Up the Update Server

1. Host the update artifacts on a server.
2. Create an update manifest file:

```json
{
  "version": "0.2.0",
  "notes": "New features and bug fixes",
  "pub_date": "2023-10-15T19:25:57Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "SIGNATURE_HERE",
      "url": "https://releases.quietdrop.com/windows/QuietDrop_0.2.0_x64_en-US.msi.zip"
    },
    "darwin-x86_64": {
      "signature": "SIGNATURE_HERE",
      "url": "https://releases.quietdrop.com/macos/QuietDrop_0.2.0_x64.app.tar.gz"
    },
    "linux-x86_64": {
      "signature": "SIGNATURE_HERE",
      "url": "https://releases.quietdrop.com/linux/quietdrop_0.2.0_amd64.AppImage.tar.gz"
    }
  }
}
```

3. Generate signatures for each package:

```bash
tauri signer sign --config tauri.conf.json QuietDrop_0.2.0_x64_en-US.msi.zip
```

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

**Solution**: Ensure the `distDir` in `tauri.conf.json` points to the correct directory containing your built frontend.

#### Android Build Errors

```
Error: Android SDK not found
```

**Solution**: Set the `ANDROID_HOME` environment variable to your Android SDK location.

```
Error: Failed to find NDK
```

**Solution**: Set the `ANDROID_NDK_HOME` environment variable or install the NDK through Android Studio.

#### iOS Build Errors

```
Error: Xcode not found
```

**Solution**: Install Xcode from the App Store and run `xcode-select --install`.

```
Error: Code signing identity not found
```

**Solution**: Set up your development team ID in `tauri.conf.json` and configure certificates in Xcode.

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
   - iOS: Use Xcode's Console app or `xcrun simctl`

3. **Inspect the built artifacts**:
   ```bash
   # Check if WebAssembly files are included
   ls -la quietdrop-tauri/dist/
   
   # Verify Tauri resources
   ls -la quietdrop-tauri/src-tauri/target/release/bundle/
   
   # Check Android build outputs
   ls -la quietdrop-tauri/gen/android/app/build/outputs/apk/
   
   # Check iOS build outputs
   ls -la quietdrop-tauri/gen/ios/App/build/Products/
   ```

4. **Run with debug logging**:
   ```bash
   # Desktop
   RUST_LOG=debug cargo tauri dev
   
   # Android
   RUST_LOG=debug cargo tauri android dev
   
   # iOS
   RUST_LOG=debug cargo tauri ios dev
   ```

## Platform-Specific Considerations

### Desktop Considerations

1. **Windows**
   - Consider using code signing certificates for better user experience and security
   - Test on different Windows versions (10, 11)
   - Handle UAC elevation gracefully

2. **macOS**
   - Configure App Sandbox properly
   - Ensure notarization for distribution outside App Store
   - Test on Apple Silicon and Intel processors

3. **Linux**
   - Support multiple package formats (deb, AppImage, etc.)
   - Test on popular distributions (Ubuntu, Fedora, etc.)
   - Handle platform-specific filesystem paths

### Mobile Considerations

1. **Android**
   - Handle different screen sizes and resolutions
   - Manage battery usage efficiently
   - Implement proper permissions requests
   - Consider Play Store requirements and guidelines

2. **iOS**
   - Adhere to App Store Review Guidelines
   - Handle app background/foreground transitions
   - Implement proper permissions requests
   - Consider screen sizes for different iPhone/iPad models

### Cross-Platform Consistency

1. **UI/UX Consistency**
   - Adjust layouts based on platform but maintain brand identity
   - Implement platform-specific UI patterns when appropriate
   - Test user flows on all platforms

2. **Feature Parity**
   - Document any platform-specific limitations
   - Provide alternative workflows when a feature isn't available on all platforms
   - Clearly communicate platform differences to users

## Best Practices

1. **Version Control**: Keep all build configuration in version control.
2. **Automated Builds**: Use CI/CD for consistent builds across platforms.
3. **Testing**: Test the packaged application, not just the development build.
4. **Incremental Updates**: Use small, frequent updates rather than infrequent large ones.
5. **Security**: Always sign your releases and verify signatures during updates.
6. **Documentation**: Keep documentation up to date with each release.
7. **Backup**: Securely back up your signing keys and certificates.
8. **Beta Testing**: Implement beta channels for both desktop and mobile versions.

### Mobile-Specific Best Practices

1. **Responsive Design**: Ensure the UI works well on all screen sizes.
2. **Battery Optimization**: Minimize network calls and background processing.
3. **Offline Support**: Implement robust offline functionality.
4. **Network Awareness**: Detect and adapt to varying network conditions.
5. **Storage Management**: Be conscious of limited storage on mobile devices.

## Distribution Channels

### Desktop Distribution

1. **Direct Download**: Offer installers on your website
2. **Package Managers**: Distribute through platform-specific package managers
   - Windows: Microsoft Store, Chocolatey, Winget
   - macOS: App Store, Homebrew
   - Linux: Snapcraft, Flathub, distro-specific repositories

### Mobile Distribution

1. **Android Distribution Channels**:
   - Google Play Store (primary)
   - F-Droid (for open source)
   - Direct APK downloads
   - Alternative app stores

2. **iOS Distribution**:
   - Apple App Store (primary)
   - TestFlight for beta testing
   - Enterprise distribution for organizational use

### Self-Updating Mechanism

For desktop applications, you can implement a self-updating mechanism using the Tauri updater:

```rust
// Check for updates programmatically
#[tauri::command]
async fn check_for_updates(window: Window) -> Result<(), String> {
    #[cfg(desktop)]
    {
        tauri::async_runtime::spawn(async move {
            match tauri::updater::builder(window.app_handle()).check().await {
                Ok(update) => {
                    if update.is_update_available() {
                        // Notify user about the update
                        window.emit("update-available", update).expect("failed to emit event");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to check for updates: {}", e);
                }
            }
        });
    }
    Ok(())
}
```

## Conclusion

This guide covers the essential steps for building, packaging, and distributing QuietDrop across different platforms using Tauri 2.0. By following these instructions, you can create reliable release builds for desktop and mobile platforms and maintain an effective update workflow.

For further assistance or to report build issues, please submit a GitHub issue or contact the project maintainers.