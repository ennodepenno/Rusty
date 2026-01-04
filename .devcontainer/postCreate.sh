#!/usr/bin/env bash
set -euo pipefail

RUST_TOOLCHAIN="stable"
DEBIAN_PACKAGES="pkg-config libssl-dev"

if [[ -n "${DEBIAN_PACKAGES// /}" ]]; then
  sudo apt-get update
  # shellcheck disable=SC2086
  sudo apt-get install -y --no-install-recommends ${DEBIAN_PACKAGES}
fi

rustup toolchain install "${RUST_TOOLCHAIN}"
rustup default "${RUST_TOOLCHAIN}"

# Warm up rust-analyzer / deps a bit.
cargo --version
rustc --version


