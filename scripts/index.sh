#!/usr/bin/env bash

# Install ckbtc locally as documented in:
# https://github.com/demergent-labs/azle/tree/main/examples/ckbtc

IC_VERSION=d6d395a480cd6986b4788f4aafffc5c03a07e46e

CKBTC_ID=mxzaz-hqaaa-aaaar-qaada-cai
INDEX_ID=n5wcd-faaaa-aaaar-qaaea-cai

mkdir -p wasm/index/
curl -o wasm/index/index.wasm.gz "https://download.dfinity.systems/ic/$IC_VERSION/canisters/ic-icrc1-index.wasm.gz"
gunzip -f wasm/index/index.wasm.gz
curl -o wasm/index/index.did "https://raw.githubusercontent.com/dfinity/ic/$IC_VERSION/rs/rosetta-api/icrc1/index/index.did"

# Deploy index
dfx deploy index --specified-id "$INDEX_ID" --argument "(record { ledger_id = principal \"$CKBTC_ID\" })"
