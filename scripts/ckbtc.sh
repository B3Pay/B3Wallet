#!/usr/bin/env bash

# Install ckbtc locally as documented in:
# https://github.com/demergent-labs/azle/tree/main/examples/ckbtc

IC_VERSION=d6d395a480cd6986b4788f4aafffc5c03a07e46e

curl -o wasm/ckbtc/ledger.wasm.gz "https://download.dfinity.systems/ic/$IC_VERSION/canisters/ic-icrc1-ledger.wasm.gz"
gunzip -f wasm/ckbtc/ledger.wasm.gz
curl -o wasm/ckbtc/ledger.did "https://raw.githubusercontent.com/dfinity/ic/$IC_VERSION/rs/rosetta-api/icrc1/ledger/ledger.did"

# Deploy ckbtc
dfx deploy ckbtc --specified-id=be2us-64aaa-aaaaa-qaabq-cai --argument='(variant { Init = record { minting_account = record { owner = principal "bd3sg-teaaa-aaaaa-qaaba-cai" }; transfer_fee = 0 : nat64; token_symbol = "ckBTC"; token_name = "ckBTC"; metadata = vec {}; initial_balances = vec {}; archive_options = record { num_blocks_to_archive = 0 : nat64; trigger_threshold = 0 : nat64; controller_id = principal "aaaaa-aa" } } })'

curl -o wasm/kyt/kyt.wasm.gz "https://download.dfinity.systems/ic/$IC_VERSION/canisters/ic-ckbtc-kyt.wasm.gz"
gunzip -f wasm/kyt/kyt.wasm.gz
curl -o wasm/kyt/kyt.did "https://raw.githubusercontent.com/dfinity/ic/$IC_VERSION/rs/bitcoin/ckbtc/kyt/kyt.did"

# Deploy kyt (know your token)
dfx deploy kyt --specified-id bkyz2-fmaaa-aaaaa-qaaaq-cai --argument "(variant { InitArg = record { minter_id = principal \"bd3sg-teaaa-aaaaa-qaaba-cai\"; maintainers = vec { principal \"$(dfx identity get-principal)\" }; mode = variant { AcceptAll } } })"
dfx canister call kyt set_api_key '(record { api_key = "" })'


curl -o wasm/minter/minter.wasm.gz "https://download.dfinity.systems/ic/$IC_VERSION/canisters/ic-ckbtc-minter.wasm.gz"
gunzip -f wasm/minter/minter.wasm.gz
curl -o wasm/minter/minter.did "https://raw.githubusercontent.com/dfinity/ic/$IC_VERSION/rs/bitcoin/ckbtc/minter/ckbtc_minter.did"

# Deploy minter
dfx deploy minter --specified-id bd3sg-teaaa-aaaaa-qaaba-cai --argument '(variant { Init = record {btc_network = variant { Regtest }; min_confirmations=opt 1; ledger_id = principal "be2us-64aaa-aaaaa-qaabq-cai"; kyt_principal = opt principal "bkyz2-fmaaa-aaaaa-qaaaq-cai"; ecdsa_key_name = "dfx_test_key";retrieve_btc_min_amount = 5_000; max_time_in_queue_nanos = 420_000_000_000; mode = variant {GeneralAvailability}} })'
