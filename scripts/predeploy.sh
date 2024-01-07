#!/usr/bin/env bash
set -eu

backend_dir="./backend"
target_dir="./target/wasm32-unknown-unknown/release"

yellow='\033[1;33m'
green='\033[0;32m'
red='\033[0;31m'
blue='\033[0;34m'
no_color='\033[0m'

# Check if a specific package name is provided
specified_package=${1:-}

# Check if the name is valid
if [ -n "$specified_package" ] && [ ! -d "$backend_dir/$specified_package" ]; then
    printf "${red}No package named $specified_package found in $backend_dir. Skipping building $specified_package.${no_color}\n"
    exit 0
fi

for app_root in "$backend_dir"/*; do
    package=$(basename "$app_root")
    # Skip packages that do not match the specified package (if one is specified)
    if [ -n "$specified_package" ] && [ "$specified_package" != "$package" ]; then
        continue
    fi

    did_file="$app_root/$package.did"
    optimised_target_dir="./wasm/$package"


    if [ ! -f "$app_root/Cargo.toml" ]; then
        printf "${yellow}No Cargo.toml found in $app_root. Skipping $package.${no_color}\n"
        continue
    fi
    
    printf "${green}Building $package in $app_root${no_color}\n"
    cargo build --manifest-path="$app_root/Cargo.toml" \
        --target wasm32-unknown-unknown \
        --release \
        --package "$package"
    printf "Size of $package.wasm: $(ls -lh "$target_dir/$package.wasm" | awk '{print $5}')\n"

    if command -v candid-extractor >/dev/null 2>&1; then
        printf "${green}Generating Candid file for $package${no_color}\n"
        candid-extractor "$target_dir/$package.wasm" 2>/dev/null > "$did_file"
        printf "Size of $package.did: $(ls -lh "$did_file" | awk '{print $5}')\n"
    else
        printf "${yellow}candid-extractor not found. Skipping generating $package.did.${no_color}\n"
    fi

    # Check if ic-wasm is installed before attempting to shrink the wasm file
    # you can install ic-wasm via `cargo install ic-wasm` for smaller wasm files
    if command -v ic-wasm >/dev/null 2>&1; then
        # create the optimised target dir
        mkdir -p "$optimised_target_dir"
        # copy the candid file to the optimised target dir
        cp "$did_file" "$optimised_target_dir/$package.did"

        # add candid file into wasm file as metadata
        printf "${green}Adding Candid file into $package.wasm${no_color}\n"
        ic-wasm "$target_dir/$package.wasm" -o "$optimised_target_dir/$package.wasm" metadata candid:service -f "$optimised_target_dir/$package.did" -v public
        printf "Size of $package.wasm with Candid metadata: $(ls -lh "$optimised_target_dir/$package.wasm" | awk '{print $5}')\n"
        
        # shrink wasm file
        printf "${green}Shrinking $package.wasm${no_color}\n"
        ic-wasm "$optimised_target_dir/$package.wasm" -o "$optimised_target_dir/$package.wasm" optimize O3
        printf "Size of shrunk $package.wasm: $(ls -lh "$optimised_target_dir/$package.wasm" | awk '{print $5}')\n"
        
        # Gunzip target
        printf "${green}Gunzipping $package.wasm${no_color}\n"
        gzip -c "$optimised_target_dir/$package.wasm" > "$optimised_target_dir/$package.wasm.gz"
        printf "Size of Gunzipped $package.wasm.gz: $(ls -lh "$optimised_target_dir/$package.wasm.gz" | awk '{print $5}')\n"
    else
        printf "${yellow}ic-wasm not found. Skipping shrinking $package.${no_color}\n"
    fi
    # print the file directory for the package
    printf "${blue}Files for $package are in $optimised_target_dir${no_color}\n"

    dfx generate "$package"

done
