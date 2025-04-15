//! Validation library for types that implement Facet
//!
//! This crate provides validation for types that implement the Facet trait,
//! leveraging reflection capabilities to perform validation without requiring
//! additional derive macros.
///
/// Example usage of facet-validate
///
/// ```rust
/// use facet::Facet;
/// use facet_validate::validate_json;
///
/// #[derive(Debug, Facet)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let json = r#"{"name": "Alice", "age": 30}"#;
/// let result = validate_json::<Person>(json);
/// assert!(result.is_empty());
///
/// let invalid_json = r#"{"name": "Bob", "age": "not a number"}"#;
/// let result = validate_json::<Person>(invalid_json);
/// assert!(!result.is_empty());
/// ```
mod error;
mod json;
mod validation;

// Re-export key components
pub use error::{ValidationError, ValidationResult, has_errors};
pub use json::{validate_json, validate_json_against_shape};
pub use validation::validate;
