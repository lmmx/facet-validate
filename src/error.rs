//! Error types for validation

/// Represents a validation error with path information
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Path to the field with the error (e.g., "user.address.street")
    pub path: String,
    /// Error message describing the validation failure
    pub message: String,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
        }
    }

    /// Create a new error at the root path
    pub fn root(message: impl Into<String>) -> Self {
        Self::new("", message)
    }
}

/// Collection of validation errors
pub type ValidationResult = Vec<ValidationError>;

/// Check if a validation result contains errors
pub fn has_errors(result: &ValidationResult) -> bool {
    !result.is_empty()
}