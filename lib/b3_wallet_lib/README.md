
# Overview
This library enables Internet Computer canisters to sign transactions for EVM-compatible blockchains.

This is a two-part process:

1. Create new addresses on the Internet Computer using distributed ECDSA  key generation
2. Sign transactions with these addresses as the canister itself or in behalve of the canister users using threshold ECDSA signatures

There an example project at [ic-evm-sign-starter](https://github.com/nikolas-con/ic-evm-sign-starter).

# Features
- Create EVM addresses
- Sign EVM transactions
- Manages transaction nonce
- Supports different chain ids
- Takes care of various tx types

**Supported Tx Types:** Legacy, EIP1559, EIP2930

# Getting Started

### Starter Project 

You can get start quickly with [ic-evm-sign-starter](https://github.com/nikolas-con/ic-evm-sign-starter).

### Existing Project

#### 1. Install the library 

Run `cargo add ic-evm-sign` in your project

#### 2. Create new EVM address

Create a new EVM-compatible address from a canister

```rust
use ic_evm_sign;

#[update]
async fn create_address() -> Result<String, String> {

    let principal_id = ic_cdk::caller();
    let response = ic_evm::create_address(principal_id).await
        .map_err(|e| format!("Failed to create address {}", e))
        .unwrap();

    Ok(response.address)
}
```

Test locally with:

```sh
dfx canister call ${canister_name} create_address
```

#### 3. Sign EVM transaction

Sign an EVM-compatible transaction from a canister

```rust
use ic_evm_sign;

#[update]
async fn sign_tx(hex_raw_tx: Vec<u8>) -> Result<String, String> {

    let chain_id = 1;
    let principal_id = ic_cdk::caller();
    let response = ic_evm_sign::sign_transaction(hex_raw_tx, chain_id, principal_id).await
        .map_err(|e| format!("Failed to sign transaction {}", e))
        .unwrap();

    Ok(response.sign_tx)
}
```

Test it locally with:

```sh
dfx canister call ${canister_name} sign_tx '(vec {${hex_raw_tx}}: vec nat8)'
```

<br/>

#### Example:

```sh
dfx canister call ${canister_name} sign_tx '(vec {236; 128; 133; 5; 66; 135; 40; 189; 130; 117; 48; 148; 112; 153; 121; 112; 197; 24; 18; 220; 58; 1; 12; 125; 1; 181; 14; 13; 23; 220; 121; 200; 136; 13; 224; 182; 179; 167; 100; 0; 0; 0; 128; 128; 128}: vec nat8)'
```

For transaction hex: 

`0xec808505428728bd8275309470997970c51812dc3a010c7d01b50e0d17dc79c8880de0b6b3a764000000808080`

# How it works

### New Address

1. Receive principal from the canister
2. Creates a new ECDSA public key on IC
3. Calculates the EVM address from the public key
4. Saves the new address to the canister state based on principal

### Transaction Signing

1. Receive a raw transaction, chain id and a principal from the canister
2. Gets principal's public key from the canister state
3. Prepares "message" to sign from raw transaction and chain id
4. Signs "message" to sign and gets transaction signature
5. Calculates recovery id from "message" to sign, signature and public key
6. And then gets the signed transaction from raw transaction, chain id and recovery id
7. Stores the transaction to the canister state based on principal

# How to's & guides

### Different EVMs

Use a different EVM-compatible blockchain using `chain_id` in:

```rust
ic_evm_sign::sign_transaction(hex_raw_tx, chain_id, principal_id)
```

Find chain ids at: [https://chainlist.org](https://chainlist.org)

### Transaction types

You can sign different transaction types hex by passing their corresponding hex using `hex_raw_tx` in:

```rust
ic_evm_sign::sign_transaction(hex_raw_tx, chain_id, principal_id)
```

Find transaction types at: [https://github.com/ethereum/execution-specs](https://github.com/ethereum/execution-specs/blob/master/lists/signature-types/README.md)

# Contributing

### Get started

1. Download the repo with `git clone`
2. Run unit tests with `cargo test`

#### To run the e2e tests:

1. Install dependencies with `npm i` in `e2e/tests`
2. Then run the e2e script with `make e2e-test`

# Security

The code in this library has not been audited (as of 13/1/22). Use it at your own risk. The same applies for [ic-evm-sign-starter](https://github.com/nikolas-con/ic-evm-sign-starter).

# Funding

This library was initially incentivized by [ICDevs](https://icdevs.org/).
