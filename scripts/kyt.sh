#!/usr/bin/env bash

# Install ckbtc locally as documented in:
# https://github.com/demergent-labs/azle/tree/main/examples/ckbtc

IC_VERSION=d6d395a480cd6986b4788f4aafffc5c03a07e46e

CKBTC_ID=mxzaz-hqaaa-aaaar-qaada-cai
KYY_ID=bkyz2-fmaaa-aaaaa-qaaaq-cai
MINTER_ID=mqygn-kiaaa-aaaar-qaadq-cai

curl -o wasm/kyt/kyt.wasm.gz "https://download.dfinity.systems/ic/$IC_VERSION/canisters/ic-ckbtc-kyt.wasm.gz"
gunzip -f wasm/kyt/kyt.wasm.gz
curl -o wasm/kyt/kyt.did "https://raw.githubusercontent.com/dfinity/ic/$IC_VERSION/rs/bitcoin/ckbtc/kyt/kyt.did"

# Deploy kyt (know your token)
dfx deploy kyt --specified-id "$KYY_ID" --argument "(variant { InitArg = record { minter_id = principal \"$MINTER_ID\"; maintainers = vec { principal \"$(dfx identity get-principal)\" }; mode = variant { AcceptAll } } })"
dfx canister call kyt set_api_key '(record { api_key = "" })'
