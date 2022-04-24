#!/bin/sh
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name wasm_osmeta --out-dir wasm/target --target web target/wasm32-unknown-unknown/release/osmeta.wasm
