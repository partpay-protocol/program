#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

//! PartPay Program is a Solana program that implements BNPL functionality
//! and equipment marketplace features

pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

solana_program::declare_id!("PARTPAY_PROGRAM_ID");

/// Export current sdk version
pub use solana_program;