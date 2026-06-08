#!/usr/bin/env bash
set -e

echo "[sign] Signing OS release..."

IMAGE="target/os_image/os.img"
SIGNATURE="target/os_image/os.img.sig"

# Placeholder signing key (replace with real PKI / secure enclave signing later)
KEY="dev_signing_key"

# Fake hash-based signature (replace with ed25519 / rsa-pss)
sha256sum "$IMAGE" | awk '{print $1}' > "$SIGNATURE"

echo "[sign] Signature written to $SIGNATURE"
echo "[sign] Using key: $KEY (placeholder)"
