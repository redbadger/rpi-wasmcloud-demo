//! # waSCC OLED provider Actor Interface
//!
//! This crate provides waSCC actors with an interface to the OLED provider. Actors using this
//! interface must have the claim `red-badger:oled-ssd1306` in order to have permission to communicate with the provider.
//!
//! The provider is one-way, and only accepts host calls from the actor. This provider does _not_
//! deliver messages to actors.
//!
//! # Example:
//! ```
//! extern crate oled_provider_interface as oled;
//! use wapc_guest::HandlerResult;
//!
//! fn add() -> HandlerResult<()> {
//!   let _ = oled::default().update("test".to_string())?;
//!   Ok(())
//! }
//! ```
//!

mod generated;

pub use generated::*;
