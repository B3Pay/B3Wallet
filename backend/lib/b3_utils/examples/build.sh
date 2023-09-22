#!/usr/bin/env bash
set -e

name="$1"
package="$2"
app_root="$(dirname "$0")/$name"
did_file="$app_root/$package.did"

# This script generates the did file, build the project (passed as $1) and then run the ic-wasm to shrink and attach metadata.
cargo build --manifest-path="$app_root/Cargo.toml" \
    --target wasm32-unknown-unknown \
    --release \
    --package "$package"

candid-extractor "./target/wasm32-unknown-unknown/release/$package.wasm" 2>/dev/null > $did_file

ic-wasm "./target/wasm32-unknown-unknown/release/$package.wasm" \
    -o "./target/wasm32-unknown-unknown/release/$package.wasm" \
    metadata candid:service -v public -f $did_file

ic-wasm "./target/wasm32-unknown-unknown/release/$package.wasm" \
    -o "./target/wasm32-unknown-unknown/release/$package-opt.wasm" \
    shrink
