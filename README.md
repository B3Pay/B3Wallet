# B3Wallet - A MultiChain and MultiOwner Wallet

## Introduction

B3Wallet is a decentralized multi-chain and multi-owner wallet. It is designed to support multiple blockchains, including Bitcoin, Ethereum, and Internet Computer. It also supports multiple owners, including single owner, multi-owner, and multi-signature. In addition, it supports multiple accounts. Being decentralized, it does not rely on any centralized service and users can use it without any registration and recover it without any backup.

## Features

- Multiple blockchain support: Bitcoin, Ethereum, and Internet Computer
- Multiple owner support: single-owner, multi-owner, and multi-signature
- Multiple account support
- Decentralized: does not rely on any centralized service, users can use it without any registration and recover it without any backup.

# Demo

![Alt text](/images/demo.png?raw=true "Demo")

## Login:

To start, log in using your Internet Identity.

![Alt text](/images/login.png "Login")

## Create Wallet:

You can create a wallet or use the UI anonymously.

To create a wallet, follow these steps:

1. Choose the wallet type: 'b3_basic_wallet' (single owner) or 'b3wallet' (multi-signature).
2. You can create a new canister or use an existing one.

- - You can create your own canister using the NNS dApp. Once it's created, copy the canister ID into the input box.
- - Alternatively, you can use the 'Create' button to generate a new canister. As of now, this is free, but it may cost some cycles in the future.
- - If you're part of a multi-signature wallet, you can add a canister ID that's already been created by other users, and click Add Canister button.

![Alt text](/images/create-wallet.png?raw=true "Create Wallet")

## Add Account:

By default, the wallet creates an account for you. However, if you need more, you can add additional accounts. These accounts are derived from the same canister but utilize different indices.

![Alt text](/images/add-account.png?raw=true "Add Account")

## Multiple Signatures:

Wallets can have multiple signers, and users have the ability to add or remove signers. Once a signer is added, a 2/2 signature consensus is required for transaction approval. The logic operates on a 50%+1 basis, so if there are three signers, a 2/3 signature consensus is required for transaction approval.

![Alt text](/images/multisig.png?raw=true "Multi-Signature")

## Self Custody:

B3Wallet provides self-custody functionality, meaning users maintain exclusive control over their canisters, and therefore, their funds. This decentralized approach to asset management aligns with the ethos of blockchain technology, enabling users to truly 'be their own bank'.

![Alt text](/images/selfcustody.png?raw=true "Self Custody")

## Self Upgrade:

B3Wallet allows for self-upgrades, ensuring that you always have access to the latest features and security updates.

![Alt text](/images/selfupgrade.png?raw=true "Self Custody")

# Custom Wallet Creation Libraries

Also, there are three libraries to facilitate other developers in building their own wallets seamlessly. These libraries, located in the [lib directory](https://github.com/B3Pay/b3wallet/tree/main/backend/lib), aim to streamline the wallet creation process and allow developers to customize a wallet that meets their unique needs.

## The Libraries:

- `b3_utils`: This foundational library provides core functionalities essential for every wallet. It equips developers with fundamental tools to initiate and manage various wallet operations.

- `operations`: This library is designed to handle the authorization and permissions within the wallet. It simplifies the process of assigning and managing permissions for different wallet operations, enabling you to create wallets that support various levels of user access.

- `b3wallet_lib`: This comprehensive library ties everything together and manages key aspects of wallet functionality. It handles account creation, key management, balance queries, and transfer processes for different blockchains and tokens. It also takes care of derivation processes, providing a versatile suite of wallet management tools. By using `b3wallet_lib`, developers can interact with a clear and convenient interface, making the creation, management, and interaction with wallets a seamless experience.

By utilizing these libraries, developers can create customized wallets tailored to their specific needs and innovative concepts.

## Use Cases

The `b3wallet` presents numerous practical applications in everyday life and the business world:

1. **Couples' Saving Wallet**: Couples can create a shared wallet to manage joint expenses or save for common goals, providing an intuitive and transparent way to handle shared finances.

2. **Company Payroll**: Businesses can leverage this wallet to distribute salaries to their employees. With the capability to handle different blockchains and tokens, it can cater to diverse payment preferences and requirements.

3. **Multi-owner Operations**: Any group with shared financial interests, from small project teams to large organizations, can use this wallet to manage their funds. By setting the required number of signatures, the group can enforce collective decision-making on financial matters.

4. **Personal Finance**: Individuals can use this wallet to manage their crypto assets across multiple blockchains from a single interface. With its robust security features and easy recovery options, it offers a reliable and user-friendly tool for personal asset management.

5. **Custom Wallets**: With the help of `b3wallet_lib`, `b3_helper`, and `b3_permit`, developers can create custom wallet solutions tailored to specific needs. This opens up a world of possibilities for innovative financial products and services built on top of the `b3wallet`.

This project harnesses the power of the Internet Computer blockchain. By utilizing the blockchain's features such as scalability, security, and seamless interoperability, `b3wallet` is able to provide a decentralized, robust, and versatile solution for managing crypto assets.

As the digital economy continues to grow, the `b3wallet` has the potential to play a crucial role in various sectors, making crypto asset management more accessible and secure for all, with the incredible capabilities of the Internet Computer blockchain backing it.

## Future Plans

I am considering the creation of a global wallet - a wallet that no one owns but everyone can sign. This revolutionary concept aims to bring the ethos of decentralization in blockchain technology to the next level. Stay tuned for more updates on this.

# Setting Up and Running B3Wallet Locally

Clone the repo : `git clone --recurse-submodules https://github.com/B3Pay/b3wallet.git `.(b3_utils is a submodule)

Follow these instructions to set up and run the B3Wallet project on your local machine:

All of these commands can also be run with npm by replacing `yarn` with `npm run` in each command.

Please make sure to have the necessary software installed and configured properly on your local machine before following these steps, including Node.js, Yarn (or npm), and DFINITY's SDK, dfx.

1. **Start the dfx service**: Open a terminal and run the following command:

```bash
yarn dfx:start
```

You can also run the command with the `--enable-bitcoin` flag if you want to enable Bitcoin:

```bash
yarn dfx:start --enable-bitcoin
```

2. **Install dfx**: While keeping the dfx service running, open another terminal and run:

   `Note: make sure you have the latest version of dfx installed and nns extension is installed, you can install it by running the following command:`
   dfx extension install nns

```bash
yarn dfx:install
```

Similar to the previous step, you can also run the command with the `--enable-bitcoin` flag:

```bash
yarn dfx:install --enable-bitcoin
```

3. **Install project dependencies**: Install all the project dependencies with:

```bash
yarn install
```

4. **Deploy the project**: Deploy the project with:

```bash
yarn deploy
```

Note: If you get `The Replica returned an error: code 3, message: "Canister bkyz2-fmaaa-aaaaa-qaaaq-cai is already installed"`, run `yarn deploy` again.

5. **Generate Candid and Types**: Generate the Candid interface and Typescript types for your canisters:

```bash
yarn generate
```

6. **Load the WASM module to system canister**: Load the WASM module to the system canister with:

```bash
yarn load-wasm:system
```

7. **Start the development server**: Finally, you can start the development server with:

```bash
yarn dev
```

8. **Start the development server**: Alternatively, you can autobuild with:

```bash
yarn autobuild
```
