# B3Wallet - A MultiChain and MultiOwner Wallet

## Introduction

B3Wallet is a decentralized multi-chain and multi-owner wallet. It is designed to support multiple blockchains, including Bitcoin, Ethereum, and Internet Computer. It also supports multiple owners, including single owner, multi-owner, and multi-signature. In addition, it supports multiple accounts, including single account. Being decentralized, it does not rely on any centralized service and users can use it without any registration and recover it without any backup.

## Features

- Multiple blockchain support: Bitcoin, Ethereum, and Internet Computer
- Multiple owner support: single owner, multi-owner, and multi-signature
- Multiple account support: single account
- Decentralized: does not rely on any centralized service, users can use it without any registration and recover it without any backup.

## Getting Started

### Setting Up and Running B3Wallet Locally

Follow these instructions to set up and run the B3Wallet project on your local machine:

1. **Start the dfx service**: Open a terminal and run the following command:

```

yarn dfx:start

```

You can also run the command with the `--enable-bitcoin` flag if you want to enable Bitcoin:

```

yarn dfx:start --enable-bitcoin

```

2. **Install dfx**: While keeping the dfx service running, open another terminal and run:

```

yarn dfx:install

```

Similar to the previous step, you can also run the command with the `--enable-bitcoin` flag:

```

yarn dfx:install --enable-bitcoin

```

3. **Install project dependencies**: Install all the project dependencies with:

```

yarn install

```

4. **Deploy the project**: Deploy the project with:

```

yarn deploy

```

5. **Generate Candid and Types**: Generate the Candid interface and Typescript types for your canisters:

```

yarn generate

```

6. **Load the WASM module to system canister**: Load the WASM module to the system canister with:

```

yarn load-wasm:system

```

7. **Start the development server**: Finally, you can start the development server with:

```

yarn dev

```

All of these commands can also be run with npm by replacing `yarn` with `npm run` in each command.

Please make sure to have the necessary software installed and configured properly on your local machine before following these steps, including Node.js, Yarn (or npm), and DFINITY's SDK, dfx.

## Demo

![Alt text](/images/demo.png?raw=true "Demo")

# Multiple Signatures:

![Alt text](/images/multisig.png?raw=true "Multi-Signature")

# Self Upgrade:

![Alt text](/images/selfupgrade.png?raw=true "Self Custody")
