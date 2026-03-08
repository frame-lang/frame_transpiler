#!/usr/bin/env bash
# Simple wrapper to invoke the framec CLI via cargo run for local testing.
# Usage: scripts/framec.sh [framec-args...]
set -euo pipefail
# Suppress rustc warnings to keep runner output focused on validator diagnostics
export RUSTFLAGS="-Awarnings"
exec cargo run -q -p framec -- "$@"
