/*!
Error handling for Lodash-RS.

This module provides comprehensive error types and handling for all operations
in the Lodash-RS library.
*/

/// Main error type for Lodash-RS operations.
#[derive(Debug, Clone, PartialEq)]
pub enum LodashError {
    /// Invalid input data
    InvalidInput { message: String },

    /// Type conversion error
    TypeConversion { from: String, to: String },

    /// Index out of bounds
    IndexOutOfBounds { index: usize, size: usize },

    /// Empty collection operation
    EmptyCollection,

    /// Invalid predicate function
    InvalidPredicate { message: String },

    /// Async operation error
    #[cfg(feature = "async")]
    AsyncError { message: String },

    /// Parallel operation error
    #[cfg(feature = "parallel")]
    ParallelError { message: String },

    /// WASM operation error
    #[cfg(feature = "wasm")]
    WasmError { message: String },

    /// Custom error from user-provided functions
    Custom { message: String },
}

impl std::fmt::Display for LodashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LodashError::InvalidInput { message } => {
                write!(f, "Invalid input: {}", message)
            }
            LodashError::TypeConversion { from, to } => {
                write!(f, "Type conversion failed: {} -> {}", from, to)
            }
            LodashError::IndexOutOfBounds { index, size } => {
                write!(f, "Index {} is out of bounds for collection of size {}", index, size)
            }
            LodashError::EmptyCollection => {
                write!(f, "Operation requires non-empty collection")
            }
            LodashError::InvalidPredicate { message } => {
                write!(f, "Invalid predicate function: {}", message)
            }
            #[cfg(feature = "async")]
            LodashError::AsyncError { message } => {
                write!(f, "Async operation failed: {}", message)
            }
            #[cfg(feature = "parallel")]
            LodashError::ParallelError { message } => {
                write!(f, "Parallel operation failed: {}", message)
            }
            #[cfg(feature = "wasm")]
            LodashError::WasmError { message } => {
                write!(f, "WASM operation failed: {}", message)
            }
            LodashError::Custom { message } => {
                write!(f, "Custom error: {}", message)
            }
        }
    }
}

impl std::error::Error for LodashError {}

/// Result type alias for Lodash-RS operations.
pub type Result<T> = std::result::Result<T, LodashError>;

impl LodashError {
    /// Create a new invalid input error.
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput {
            message: message.into(),
        }
    }

    /// Create a new type conversion error.
    pub fn type_conversion(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self::TypeConversion {
            from: from.into(),
            to: to.into(),
        }
    }

    /// Create a new index out of bounds error.
    pub fn index_out_of_bounds(index: usize, size: usize) -> Self {
        Self::IndexOutOfBounds { index, size }
    }

    /// Create a new empty collection error.
    pub fn empty_collection() -> Self {
        Self::EmptyCollection
    }

    /// Create a new invalid predicate error.
    pub fn invalid_predicate(message: impl Into<String>) -> Self {
        Self::InvalidPredicate {
            message: message.into(),
        }
    }

    /// Create a new custom error.
    pub fn custom(message: impl Into<String>) -> Self {
        Self::Custom {
            message: message.into(),
        }
    }

    #[cfg(feature = "async")]
    /// Create a new async error.
    pub fn async_error(message: impl Into<String>) -> Self {
        Self::AsyncError {
            message: message.into(),
        }
    }

    #[cfg(feature = "parallel")]
    /// Create a new parallel error.
    pub fn parallel_error(message: impl Into<String>) -> Self {
        Self::ParallelError {
            message: message.into(),
        }
    }

    #[cfg(feature = "wasm")]
    /// Create a new WASM error.
    pub fn wasm_error(message: impl Into<String>) -> Self {
        Self::WasmError {
            message: message.into(),
        }
    }
}

/// Extension trait for converting other error types to LodashError.
pub trait IntoLodashError<T> {
    /// Convert the error to a LodashError.
    fn into_lodash_error(self) -> Result<T>;
}

impl<T, E> IntoLodashError<T> for std::result::Result<T, E>
where
    E: std::fmt::Display,
{
    fn into_lodash_error(self) -> Result<T> {
        self.map_err(|e| LodashError::custom(e.to_string()))
    }
}

/// Macro for creating custom errors with context.
#[macro_export]
macro_rules! lodash_error {
    ($variant:ident, $($arg:expr),*) => {
        LodashError::$variant {
            $($arg),*
        }
    };
}

/// Macro for early return on error with context.
#[macro_export]
macro_rules! lodash_try {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err.into_lodash_error().unwrap_err()),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = LodashError::invalid_input("test message");
        assert!(matches!(err, LodashError::InvalidInput { .. }));
        assert_eq!(err.to_string(), "Invalid input: test message");

        let err = LodashError::type_conversion("String", "i32");
        assert!(matches!(err, LodashError::TypeConversion { .. }));
        assert_eq!(err.to_string(), "Type conversion failed: String -> i32");

        let err = LodashError::index_out_of_bounds(5, 3);
        assert!(matches!(err, LodashError::IndexOutOfBounds { .. }));
        assert_eq!(err.to_string(), "Index 5 is out of bounds for collection of size 3");

        let err = LodashError::empty_collection();
        assert!(matches!(err, LodashError::EmptyCollection));
        assert_eq!(err.to_string(), "Operation requires non-empty collection");
    }

    #[test]
    fn test_error_conversion() {
        let result: std::result::Result<i32, String> = Err("test error".to_string());
        let lodash_result: Result<i32> = result.into_lodash_error();
        
        assert!(lodash_result.is_err());
        if let Err(LodashError::Custom { message }) = lodash_result {
            assert_eq!(message, "test error");
        } else {
            panic!("Expected Custom error variant");
        }
    }
}
