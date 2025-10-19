/*!
Utility modules for Lodash-RS.

This module contains utility functions, type conversions, and helper traits
used throughout the library.
*/

pub mod error;
pub mod type_conv;
pub mod async_support;

// Re-export commonly used items
pub use error::{LodashError, Result, IntoLodashError};
pub use type_conv::*;

#[cfg(feature = "async")]
pub use async_support::*;
