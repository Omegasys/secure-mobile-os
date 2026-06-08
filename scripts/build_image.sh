#!/usr/bin/env bash
set -e

echo "[build] Starting OS build..."

# Build kernel (Rust)
cargo build --release

echo "[build] Compiling boot image..."

# Create output directories
mkdir -p target/os_image

# Copy kernel binary
cp target/release/kernel target/os_image/kernel.bin

# Placeholder initramfs / system image
echo "rootfs placeholder" > target/os_image/rootfs.img

# Combine into disk image
dd if=/dev/zero of=target/os_image/os.img bs=1M count=64

echo "[build] OS image created at target/os_image/os.img"
