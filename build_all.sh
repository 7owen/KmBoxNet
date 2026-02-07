#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Starting multi-platform build for KmBoxNet...${NC}"

# Ensure output directory exists
mkdir -p dist

# Check if zig is installed
if ! command -v zig &> /dev/null; then
    echo "❌ Error: 'zig' is not installed. Please run 'brew install zig' first for cross-compilation."
    exit 1
fi

# 1. macOS ARM64 (Apple Silicon)
echo -e "${GREEN}📦 Building for macOS (ARM64)...${NC}"
# Target added via: rustup target add aarch64-apple-darwin
uvx maturin build --release --target aarch64-apple-darwin --out pylib

# 2. macOS x86_64 (Intel)
echo -e "${GREEN}📦 Building for macOS (x86_64)...${NC}"
# Target added via: rustup target add x86_64-apple-darwin
uvx maturin build --release --target x86_64-apple-darwin --out pylib

# 3. Windows x86_64
echo -e "${GREEN}📦 Building for Windows (x86_64)...${NC}"
# Target added via: rustup target add x86_64-pc-windows-msvc
uvx maturin build --release --target x86_64-pc-windows-msvc --zig --out pylib

# 4. Windows ARM64
echo -e "${GREEN}📦 Building for Windows (ARM64)...${NC}"
# Target added via: rustup target add aarch64-pc-windows-msvc
uvx maturin build --release --target aarch64-pc-windows-msvc --zig --out pylib

# 5. Linux x86_64 (Manylinux compatible)
echo -e "${GREEN}📦 Building for Linux (x86_64)...${NC}"
# Target added via: rustup target add x86_64-unknown-linux-gnu
uvx maturin build --release --target x86_64-unknown-linux-gnu --zig --out pylib

# 6. Linux ARM64 (Manylinux compatible)
echo -e "${GREEN}📦 Building for Linux (aarch64)...${NC}"
# Target added via: rustup target add aarch64-unknown-linux-gnu
uvx maturin build --release --target aarch64-unknown-linux-gnu --zig --out pylib

echo -e "${BLUE}✅ All builds completed successfully!${NC}"
echo -e "${BLUE}📁 Wheels are located in the 'dist' directory:${NC}"
ls -lh dist
