#!/usr/bin/env bash
set -eu

backend_dir="./backend"
target_dir="./target/wasm32-unknown-unknown/release"

yellow='\033[1;33m'
green='\033[0;32m'
no_color='\033[0m'

for app_root in "$backend_dir"/*; do
    package=$(basename "$app_root")
    did_file="$app_root/$package.did"

    echo "${green}Building $package in $app_root${no_color}"
    cargo build --manifest-path="$app_root/Cargo.toml" \
        --target wasm32-unknown-unknown \
        --release \
        --package "$package"
    echo "Size of $package.wasm: $(ls -lh "$target_dir/$package.wasm" | awk '{print $5}')"

    if command -v candid-extractor >/dev/null 2>&1; then
        echo "${green}Generating Candid file for $package${no_color}"
        candid-extractor "$target_dir/$package.wasm" 2>/dev/null > "$did_file"
        echo "Size of $package.did: $(ls -lh "$did_file" | awk '{print $5}')"
    else
        echo "${yellow}candid-extractor not found. Skipping generating $package.did.${no_color}"
    fi

    # Check if ic-wasm is installed before attempting to shrink the wasm file
    if command -v ic-wasm >/dev/null 2>&1; then
        # you can install ic-wasm via `cargo install ic-wasm` for smaller wasm files
        echo "${green}Shrinking $package.wasm${no_color}"
        ic-wasm "$target_dir/$package.wasm" -o "$target_dir/$package.wasm" shrink
        echo "Size of shrunk $package.wasm: $(ls -lh "$target_dir/$package.wasm" | awk '{print $5}')"
    else
        echo "${yellow}ic-wasm not found. Skipping shrinking $package.${no_color}"
    fi

    dfx generate "$package"

done
