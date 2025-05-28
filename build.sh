#!/usr/bin/env bash
set -e

cd $(dirname "$0")
DIRNAME="$PWD"

cd "$DIRNAME"
cargo run --bin=bindings "$@"

cd "$DIRNAME/frontend"
npm cache clean --force
npm install
npm run build

cd "$DIRNAME"
cargo build "$@"

# To run: cargo run -- --verbose
