#!/usr/bin/env bash
set -e

IMAGE="target/os_image/os.img"

echo "[run] Launching OS in QEMU..."

qemu-system-x86_64 \
    -m 2048 \
    -drive file=$IMAGE,format=raw \
    -serial stdio \
    -display gtk \
    -cpu qemu64 \
    -smp 4
