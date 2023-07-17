//! # B3Helper Library
//!
//! `b3_helper_lib` is a Rust library designed to simplify the development of applications and wallets on the Internet Computer. It provides utility functions and types for handling various operations such as transfers, tokens, timestamps, and more.
//!
//! ## Features
//!  
//! - **Account** - Create and manage accounts.
//! - **Constants** - Constants used by the library.
//! - **Currency** - Currency types and conversion functions.
//! - **Environment** - Environment types and conversion functions.
//! - **Error** - Error types and conversion functions.
//! - **Owner** - Owner types and conversion functions.
//! - **Release** - Release types and conversion functions.
//! - **Subaccount** - Subaccount types and conversion functions.
//! - **System** - System types and conversion functions.
//! - **Timestamp** - Timestamp types and conversion functions.
//! - **Ledger** - Ledger types and conversion functions.
//! - **Utils** - Utility functions.
//! - **Wallet** - Wallet types and conversion functions.
//! - **Wasm** - Wasm types and conversion functions.
//!
//! ## Examples
//!
//! Here's a simple example of how to create a new [icrc1 account](https://internetcomputer.org/docs/current/developer-docs/integrations/icrc-1/) :
//!
//! ```rust
//! use b3_helper_lib::ICRCAccount;
//! use b3_helper_lib::Subaccount;
//! use b3_helper_lib::Environment;
//! use candid::Principal;
//!
//! let owner = Principal::from_text("b7pqa-qqaaa-aaaap-abdva-cai").unwrap();
//! let subaccount = Subaccount::new(Environment::Production, 1);
//!
//! let account = ICRCAccount::new(owner, Some(subaccount));
//!
//! assert_eq!(account.to_text(), "b7pqa-qqaaa-aaaap-abdva-cai-vpwy45i.1");
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
pub mod constants;
pub mod currency;
pub mod environment;
pub mod error;
pub mod ledger;
pub mod owner;
pub mod release;
pub mod subaccount;
pub mod system;
pub mod timestamp;
pub mod utils;
pub mod wallet;
pub mod wasm;

pub use account::*;
pub use environment::*;
pub use subaccount::*;
pub use utils::*;
