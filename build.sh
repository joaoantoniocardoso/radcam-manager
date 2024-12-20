#!/usr/bin/env bash
set -e

cd $(dirname "$0")
DIRNAME="$PWD"

cd "$DIRNAME/backend"
cargo run --bin=bindings "$@"

cd "$DIRNAME/frontend"
npm cache clean --force
npm install
npm run build

cd "$DIRNAME/backend"
cargo build "$@"

# To run: cd backend && cargo run -- --verbose
