#!/usr/bin/env bash

# Install ckbtc locally as documented in:
# https://github.com/demergent-labs/azle/tree/main/examples/ckbtc

IC_VERSION=d6d395a480cd6986b4788f4aafffc5c03a07e46e

CKBTC_ID=mxzaz-hqaaa-aaaar-qaada-cai
MINTER_ID=mqygn-kiaaa-aaaar-qaadq-cai

mkdir -p wasm/ckbtc/
curl -o wasm/ckbtc/ledger.wasm.gz "https://download.dfinity.systems/ic/$IC_VERSION/canisters/ic-icrc1-ledger.wasm.gz"
gunzip -f wasm/ckbtc/ledger.wasm.gz
curl -o wasm/ckbtc/ledger.did "https://raw.githubusercontent.com/dfinity/ic/$IC_VERSION/rs/rosetta-api/icrc1/ledger/ledger.did"

# Deploy ckbtc
dfx deploy ckbtc --specified-id "$CKBTC_ID" --argument="(variant { Init = record { minting_account = record { owner = principal \"$MINTER_ID\"}; transfer_fee = 0 : nat64; token_symbol = \""ckBTC"\"; token_name = \""ckBTC"\"; metadata = vec {}; initial_balances = vec {}; archive_options = record { num_blocks_to_archive = 0 : nat64; trigger_threshold = 0 : nat64; controller_id = principal \""aaaaa-aa"\"} } })"
