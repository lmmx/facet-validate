//! JSON-specific validation

use facet::{Facet, Shape};
use facet_json::{self, from_str};
use facet_reflect::Peek;

use crate::error::{ValidationError, ValidationResult};
use crate::validation::validate;

/// Validate JSON string against a type that implements Facet
pub fn validate_json<T: Facet>(json_str: &str) -> ValidationResult {
    // First, try to parse the JSON
    match from_str::<T>(json_str) {
        Ok(value) => {
            // Successfully parsed, now validate the structure
            let peek = Peek::new(&value);
            validate(peek, T::SHAPE, "")
        }
        Err(e) => {
            // JSON parsing or type conversion failed
            vec![ValidationError::root(format!("JSON parsing error: {}", e))]
        }
    }
}

/// Try to parse JSON without a specific target type, then validate against shape
pub fn validate_json_against_shape(_json_str: &str, _shape: &'static Shape) -> ValidationResult {
    // This function would be useful for dynamic validation scenarios
    // However, facet-json doesn't have a great way to parse to "any" type
    // For real implementation, this would need more work
    vec![ValidationError::root("Dynamic JSON validation not implemented")]
}
