#!/bin/bash
curl https://sh.rustup.rs -sSf | sh -s -- -y
PATH=$PATH:/vercel/.cargo/bin

rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos