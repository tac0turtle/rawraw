//! **WARNING: This is an API preview! Expect major bugs, glaring omissions, and breaking changes!**
//!
//! This crate provides a low-level implementation of the Cosmos SDK RFC 003 message passing API.
#![no_std]

mod account_id;
pub mod code;
pub mod data_pointer;
pub mod handler;
pub mod header;
pub mod packet;
pub mod message;

pub use account_id::AccountID;

/// The root system account ID.
pub const ROOT_ACCOUNT: AccountID = AccountID::new(1);

