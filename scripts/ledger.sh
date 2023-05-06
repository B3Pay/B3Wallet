#!/usr/bin/env bash

# Install ledger locally as documented in:
# https://internetcomputer.org/docs/current/developer-docs/integrations/ledger/ledger-local-setup

IC_VERSION=f02cc38677905e24a9016637fddc697039930808
curl -o icp/ledger/ledger.wasm.gz "https://download.dfinity.systems/ic/$IC_VERSION/canisters/ledger-canister_notify-method.wasm.gz"
gunzip icp/ledger/ledger.wasm.gz
curl -o icp/ledger/ledger.private.did "https://raw.githubusercontent.com/dfinity/ic/$IC_VERSION/rs/rosetta-api/ledger.did"
curl -o icp/ledger/ledger.public.did "https://raw.githubusercontent.com/dfinity/ic/$IC_VERSION/rs/rosetta-api/icp_ledger/ledger.did"

dfx identity new minter
dfx identity use minter
MINT_ACC=$(dfx ledger account-id)

dfx identity use default

# LEDGER_ACC=$(dfx ledger account-id)
LEDGER_ACC=ef05571645cc55dd1e5ac9984fa69494a97f5467dfe4540c867742222aecc611

dfx deploy ledger --argument '(record {minting_account = "'${MINT_ACC}'"; initial_values = vec { record { "'${LEDGER_ACC}'"; record { e8s=100_000_000_000 } }; }; send_whitelist = vec {}})'

# Rust example to transfer ICP
# https://github.com/dfinity/examples/tree/master/rust/tokens_transfer