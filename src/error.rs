use napi_derive::napi;
use std::fmt;

/// Custom error types for stdio-napi
#[derive(Debug)]
#[napi]
pub enum StdioError {
    /// I/O related errors
    Io(String),
    /// Terminal related errors
    Terminal(String),
    /// Process related errors
    Process(String),
    /// Stream related errors
    Stream(String),
    /// Validation errors
    Validation(String),
    /// Buffer related errors
    Buffer(String),
}

impl fmt::Display for StdioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StdioError::Io(msg) => write!(f, "IO Error: {}", msg),
            StdioError::Terminal(msg) => write!(f, "Terminal Error: {}", msg),
            StdioError::Process(msg) => write!(f, "Process Error: {}", msg),
            StdioError::Stream(msg) => write!(f, "Stream Error: {}", msg),
            StdioError::Validation(msg) => write!(f, "Validation Error: {}", msg),
            StdioError::Buffer(msg) => write!(f, "Buffer Error: {}", msg),
        }
    }
}

impl std::error::Error for StdioError {}

/// Result type alias using StdioError
pub type StdioResult<T> = Result<T, StdioError>;

/// Extension trait for napi::Result to convert to StdioError
pub trait NapiResultExt<T> {
    fn to_stdio_error(self, context: &str) -> napi::Result<T>;
}

impl<T> NapiResultExt<T> for napi::Result<T> {
    fn to_stdio_error(self, context: &str) -> napi::Result<T> {
        self.map_err(|e| napi::Error::from_reason(format!("{}: {}", context, e)))
    }
}

/// Validation helper functions
pub mod validation {
    /// Validate that a string is not empty
    pub fn require_non_empty(value: &str, field_name: &str) -> Result<(), String> {
        if value.trim().is_empty() {
            Err(format!("{} cannot be empty", field_name))
        } else {
            Ok(())
        }
    }

    /// Validate that a vector is not empty
    pub fn require_non_empty_vec<T>(value: &[T], field_name: &str) -> Result<(), String> {
        if value.is_empty() {
            Err(format!("{} cannot be empty", field_name))
        } else {
            Ok(())
        }
    }

    /// Validate string length is within bounds
    pub fn validate_length(
        value: &str,
        field_name: &str,
        min: Option<usize>,
        max: Option<usize>,
    ) -> Result<(), String> {
        let len = value.len();
        
        if let Some(min_len) = min {
            if len < min_len {
                return Err(format!(
                    "{} must be at least {} characters",
                    field_name, min_len
                ));
            }
        }
        
        if let Some(max_len) = max {
            if len > max_len {
                return Err(format!(
                    "{} must be at most {} characters",
                    field_name, max_len
                ));
            }
        }
        
        Ok(())
    }

    /// Validate a number is within bounds
    pub fn validate_range(
        value: u32,
        field_name: &str,
        min: Option<u32>,
        max: Option<u32>,
    ) -> Result<(), String> {
        if let Some(min_val) = min {
            if value < min_val {
                return Err(format!("{} must be at least {}", field_name, min_val));
            }
        }
        
        if let Some(max_val) = max {
            if value > max_val {
                return Err(format!("{} must be at most {}", field_name, max_val));
            }
        }
        
        Ok(())
    }
}
