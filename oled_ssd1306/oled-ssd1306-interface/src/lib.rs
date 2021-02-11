//! # Oled wasmCloud Actor Interface
//!
//! This crate provides wasmCloud actors with an interface to the Oled capability provider. Actors using this
//! interface must have the claim `red-badger:oled-ssd1306` in order to have permission to handle requests, and they
//! must have an active, configured binding to an Oled capability provider.
//!
//! # Example:
//! ```
//! extern crate oled;
//! use wapc_guest as guest;
//! use oled::generated::*;
//! use guest::prelude::*;

//! #[no_mangle]
//! pub fn wapc_init() {
//!     Handlers::register_update(update);
//! }

//! fn update(_txt: String) -> HandlerResult<UpdateResponse> {
//!     Ok(UpdateResponse::default()) // TODO: Provide implementation.
//! }
//! ```
//!

mod generated;
pub use generated::*;
