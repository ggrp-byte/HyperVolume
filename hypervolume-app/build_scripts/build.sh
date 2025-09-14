#!/bin/bash
# HyperVolume Build Script for Linux/macOS
# Note: This is primarily for development and testing on non-Windows platforms

set -e

CONFIGURATION="Release"
CREATE_INSTALLER=false
SKIP_TESTS=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --debug)
            CONFIGURATION="Debug"
            shift
            ;;
        --create-installer)
            CREATE_INSTALLER=true
            shift
            ;;
        --skip-tests)
            SKIP_TESTS=true
            shift
            ;;
        *)
            echo "Unknown option $1"
            exit 1
            ;;
    esac
done

echo "Building HyperVolume..."

# Get project root directory
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

echo "Project root: $PROJECT_ROOT"

# Install Node.js dependencies
echo "Installing Node.js dependencies..."
npm install

# Run tests if not skipped
if [ "$SKIP_TESTS" = false ]; then
    echo "Running tests..."
    cd src-tauri
    cargo test
    cd "$PROJECT_ROOT"
fi

# Build the application
echo "Building Tauri application..."
if [ "$CONFIGURATION" = "Release" ]; then
    npm run tauri build
else
    npm run tauri build -- --debug
fi

echo "Build completed successfully!"

# Note: Installer creation is Windows-specific
if [ "$CREATE_INSTALLER" = true ]; then
    echo "Warning: Installer creation is only supported on Windows with NSIS"
    echo "Please run the PowerShell build script on Windows to create the installer"
fi

