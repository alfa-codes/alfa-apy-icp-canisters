#!/bin/bash
set -e

# Build the strategy_history canister
cargo build --target wasm32-unknown-unknown --release --package strategy_history

# Copy the wasm file to the root directory
cp target/wasm32-unknown-unknown/release/strategy_history.wasm strategy_history.wasm

echo "Strategy history canister built successfully!"
