//! # B3Helper Library
//!
//! `b3_helper_lib` is a Rust library designed to simplify the development of applications and wallets on the Internet Computer. It provides utility functions and types for handling various operations such as transfers, tokens, timestamps, and more.
//!
//! ## Features
//!
//! - **Account**: Provides functions and types for handling accounts.
//! - **Amount**: Provides functions and types for handling amounts.
//! - **Constants**: Defines various constants used throughout the library.
//! - **Environment**: Provides functions and types for handling the environment.
//! - **Error**: Defines various error types used throughout the library.
//! - **Identifier**: Provides functions and types for handling identifiers.
//! - **Notify**: Provides functions for sending notifications.
//! - **Owner**: Provides functions and types for handling owners.
//! - **Release**: Provides functions and types for handling releases.
//! - **Subaccount**: Provides functions and types for handling subaccounts.
//! - **System**: Provides functions and types for handling system-level operations.
//! - **Timestamp**: Provides functions and types for handling timestamps.
//! - **Token**: Provides functions and types for handling tokens.
//! - **Transfer**: Provides functions and types for handling transfers.
//! - **Utils**: Provides various utility functions.
//! - **Wallet**: Provides functions and types for handling wallets.
//! - **WASM**: Provides functions for handling WASM uploads.
//!
//! ## Examples
//!
//! Here's a simple example of how to create a new account:
//!
//! ```rust
//! use b3_helper_lib::account::Account;
//!
//! let account = Account::new();
//! ```
//!
//! For more detailed examples, see the documentation for each module.
//!
//! ## More Information
//!
//! For more information, see the [API documentation](https://docs.rs/b3_helper_lib).
//!
//! ## Contributing
//!
//! We welcome contributions to the B3 Helper Library! Please see our [contributing guide](CONTRIBUTING.md) for more details.
//!
//! ## License
//!
//! This project is licensed under the [MIT License](LICENSE).

pub mod account;
pub mod amount;
pub mod constants;
pub mod environment;
pub mod error;
pub mod identifier;
pub mod notify;
pub mod owner;
pub mod release;
pub mod subaccount;
pub mod system;
pub mod timestamp;
pub mod token;
pub mod transfer;
pub mod utils;
pub mod wallet;
pub mod wasm;

pub use account::*;
pub use identifier::*;
pub use subaccount::*;
