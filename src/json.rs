//! JSON-specific validation

use facet::Facet;
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
