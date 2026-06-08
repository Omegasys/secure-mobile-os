#!/usr/bin/env bash
set -e

echo "[deploy] Deploying OS build to test network..."

IMAGE="target/os_image/os.img"
SIGNATURE="target/os_image/os.img.sig"

if [ ! -f "$IMAGE" ]; then
    echo "[error] OS image not found. Run build_image.sh first."
    exit 1
fi

if [ ! -f "$SIGNATURE" ]; then
    echo "[error] Signature not found. Run sign_release.sh first."
    exit 1
fi

# Simulated testnet upload
TESTNET_NODE="http://localhost:8080/upload"

echo "[deploy] Uploading image..."
curl -X POST "$TESTNET_NODE" \
    -F "image=@$IMAGE" \
    -F "signature=@$SIGNATURE"

echo "[deploy] Deployment complete."
