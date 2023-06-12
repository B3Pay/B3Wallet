#!/usr/bin/env bash

# Install ckbtc locally as documented in:
# https://github.com/demergent-labs/azle/tree/main/examples/ckbtc

IC_VERSION=d6d395a480cd6986b4788f4aafffc5c03a07e46e

CKBTC_ID=mxzaz-hqaaa-aaaar-qaada-cai
KYY_ID=bkyz2-fmaaa-aaaaa-qaaaq-cai
MINTER_ID=mqygn-kiaaa-aaaar-qaadq-cai

curl -o wasm/minter/minter.wasm.gz "https://download.dfinity.systems/ic/$IC_VERSION/canisters/ic-ckbtc-minter.wasm.gz"
gunzip -f wasm/minter/minter.wasm.gz
curl -o wasm/minter/minter.did "https://raw.githubusercontent.com/dfinity/ic/$IC_VERSION/rs/bitcoin/ckbtc/minter/ckbtc_minter.did"

# Deploy minter
dfx deploy minter --specified-id "$MINTER_ID" --argument "(variant { Init = record {btc_network = variant { Regtest }; min_confirmations=opt 1; ledger_id = principal \"$CKBTC_ID\"; kyt_principal = opt principal \"$KYY_ID\"; ecdsa_key_name = \""dfx_test_key"\";retrieve_btc_min_amount = 5_000; max_time_in_queue_nanos = 420_000_000_000; mode = variant {GeneralAvailability}} })"
