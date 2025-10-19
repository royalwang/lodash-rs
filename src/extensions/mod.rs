/*!
Extensions module for Lodash-RS.

This module provides advanced features like parallel processing, WASM support,
and other extensions that enhance the core functionality.
*/

pub mod parallel;
pub mod wasm;

// Re-export commonly used items
#[cfg(feature = "parallel")]
pub use parallel::*;

#[cfg(feature = "wasm")]
pub use wasm::*;
