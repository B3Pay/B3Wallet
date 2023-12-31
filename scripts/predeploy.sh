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
    optimised_target_dir="./wasm/$package"

    if [ ! -f "$app_root/Cargo.toml" ]; then
        echo -e "${yellow}No Cargo.toml found in $app_root. Skipping $package.${no_color}"
        continue
    fi
    
    echo -e "${green}Building $package in $app_root${no_color}"
    cargo build --manifest-path="$app_root/Cargo.toml" \
        --target wasm32-unknown-unknown \
        --release \
        --package "$package"
    echo -e "Size of $package.wasm: $(ls -lh "$target_dir/$package.wasm" | awk '{print $5}')"

    if command -v candid-extractor >/dev/null 2>&1; then
        echo -e "${green}Generating Candid file for $package${no_color}"
        candid-extractor "$target_dir/$package.wasm" 2>/dev/null > "$did_file"
        echo -e "Size of $package.did: $(ls -lh "$did_file" | awk '{print $5}')"
    else
        echo -e "${yellow}candid-extractor not found. Skipping generating $package.did.${no_color}"
    fi

    # Check if ic-wasm is installed before attempting to shrink the wasm file
    # you can install ic-wasm via `cargo install ic-wasm` for smaller wasm files
    if command -v ic-wasm >/dev/null 2>&1; then
        # create the optimised target dir
        mkdir -p "$optimised_target_dir"
        # copy the candid file to the optimised target dir
        cp "$did_file" "$optimised_target_dir/$package.did"

        # add candid file into wasm file as metadata
        echo -e "${green}Adding Candid file into $package.wasm${no_color}"
        ic-wasm "$target_dir/$package.wasm" -o "$optimised_target_dir/$package.wasm" metadata candid:service -f "$optimised_target_dir/$package.did" -v public
        echo -e "Size of $package.wasm with Candid metadata: $(ls -lh "$optimised_target_dir/$package.wasm" | awk '{print $5}')"
        
        # shrink wasm file
        echo -e "${green}Shrinking $package.wasm${no_color}"
        ic-wasm "$optimised_target_dir/$package.wasm" -o "$optimised_target_dir/$package.wasm" optimize O3
        echo -e "Size of shrunk $package.wasm: $(ls -lh "$optimised_target_dir/$package.wasm" | awk '{print $5}')"
        
        # Gunzip target
        echo -e "${green}Gunzipping $package.wasm${no_color}"
        gzip -c "$optimised_target_dir/$package.wasm" > "$optimised_target_dir/$package.wasm.gz"
        echo -e "Size of Gunzipped $package.wasm.gz: $(ls -lh "$optimised_target_dir/$package.wasm.gz" | awk '{print $5}')"
    else
        echo -e "${yellow}ic-wasm not found. Skipping shrinking $package.${no_color}"
    fi

    dfx generate "$package"

done
