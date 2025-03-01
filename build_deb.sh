#!/bin/bash

# Exit on error
set -e

# Configuration
PACKAGE_NAME="objr"
VERSION="0.1.0"
ARCH="amd64"
MAINTAINER="Your Name <your.email@example.com>"
DESCRIPTION="Objective-R compiler"

# Create temporary directory structure
BUILD_DIR="debian_build"
rm -rf $BUILD_DIR
mkdir -p $BUILD_DIR/DEBIAN
mkdir -p $BUILD_DIR/usr/local/bin

# Build release binary
echo "Building release binary..."
cargo build --release --bin objr

# Copy binary to package directory
cp target/release/objr $BUILD_DIR/usr/local/bin/

# Create control file
cat > $BUILD_DIR/DEBIAN/control << EOF
Package: $PACKAGE_NAME
Version: $VERSION
Architecture: $ARCH
Maintainer: $MAINTAINER
Description: $DESCRIPTION
 A compiler for the Objective-R programming language.
 Translates .rr files into native code.
EOF

# Make the binary executable
chmod 755 $BUILD_DIR/usr/local/bin/objr

# Build the package
dpkg-deb --build $BUILD_DIR ${PACKAGE_NAME}_${VERSION}_${ARCH}.deb

# Cleanup
rm -rf $BUILD_DIR

echo "Package created: ${PACKAGE_NAME}_${VERSION}_${ARCH}.deb"
