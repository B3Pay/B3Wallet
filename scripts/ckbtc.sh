#!/usr/bin/env bash

# Install ckbtc locally as documented in:

IC_VERSION=d6d395a480cd6986b4788f4aafffc5c03a07e46e
curl -o wasm/ckbtc/ledger.wasm.gz "https://download.dfinity.systems/ic/$IC_VERSION/canisters/ic-icrc1-ledger.wasm.gz"
gunzip wasm/ckbtc/ledger.wasm.gz
curl -o wasm/ckbtc/ledger.did "https://raw.githubusercontent.com/dfinity/ic/$IC_VERSION/rs/rosetta-api/icrc1/ledger/ledger.did"

dfx identity new minter
dfx identity use minter
MINT_ACC=$(dfx ledger account-id)

dfx identity use default

# LEDGER_ACC=$(dfx ledger account-id)
LEDGER_ACC=ef05571645cc55dd1e5ac9984fa69494a97f5467dfe4540c867742222aecc611

dfx deploy ledger --argument '(record {minting_account = "'${MINT_ACC}'"; initial_values = vec { record { "'${LEDGER_ACC}'"; record { e8s=100_000_000_000 } }; }; send_whitelist = vec {}})'

# Rust example to transfer ICP
# https://github.com/dfinity/examples/tree/master/rust/tokens_transfer