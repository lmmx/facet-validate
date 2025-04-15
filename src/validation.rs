//! Core validation functions

use facet::{Def, Shape};
use facet_reflect::Peek;

use crate::error::{ValidationError, ValidationResult};

/// Build a field path by joining parent and child paths
pub fn build_path(parent: &str, field: &str) -> String {
    if parent.is_empty() {
        field.to_string()
    } else {
        format!("{}.{}", parent, field)
    }
}

/// Validate a value against a shape, with path context
pub fn validate<'a>(
    value: Peek<'a>,
    expected_shape: &'static Shape,
    path: &str,
) -> ValidationResult {
    // First check shape compatibility
    if value.shape() != expected_shape {
        return vec![ValidationError::new(
            path,
            format!(
                "Type mismatch: expected {}, got {}",
                expected_shape,
                value.shape()
            ),
        )];
    }

    match expected_shape.def {
        Def::Struct(_) => validate_struct(value, path),
        Def::List(_) => validate_list(value, path),
        Def::Map(_) => validate_map(value, path),
        Def::Enum(_) => validate_enum(value, path),
        _ => Vec::new(), // Scalars already passed the shape check
    }
}

/// Validate a struct value
fn validate_struct<'a>(value: Peek<'a>, path: &str) -> ValidationResult {
    let mut errors = Vec::new();

    // Try to convert to struct view
    let struct_result = value.into_struct();

    match struct_result {
        Ok(struct_value) => {
            // Check each field
            for (field, field_value) in struct_value.fields() {
                let field_path = build_path(path, field.name);
                let field_errors = validate(field_value, field.shape(), &field_path);
                errors.extend(field_errors);
            }
        }
        Err(e) => {
            errors.push(ValidationError::new(
                path,
                format!("Failed to access struct: {}", e),
            ));
        }
    }

    errors
}

/// Validate a list value
fn validate_list<'a>(value: Peek<'a>, path: &str) -> ValidationResult {
    let mut errors = Vec::new();

    let list_result = value.into_list();

    match list_result {
        Ok(list_value) => {
            // Get element shape from list definition
            let element_shape = list_value.def().t();

            // Check each element
            for (index, element) in list_value.iter().enumerate() {
                let element_path = format!("{}[{}]", path, index);
                let element_errors = validate(element, element_shape, &element_path);
                errors.extend(element_errors);
            }
        }
        Err(e) => {
            errors.push(ValidationError::new(
                path,
                format!("Failed to access list: {}", e),
            ));
        }
    }

    errors
}

/// Validate a map value
fn validate_map<'a>(value: Peek<'a>, path: &str) -> ValidationResult {
    let mut errors = Vec::new();

    let map_result = value.into_map();

    match map_result {
        Ok(map_value) => {
            // Get value shape from map definition
            let value_shape = map_value.def().v;

            // Check each entry
            for (key, val) in &map_value {
                // For simplicity, we'll use key's string representation for the path
                let key_str = format!("{}", key);
                let entry_path = build_path(path, &key_str);

                let value_errors = validate(val, value_shape, &entry_path);
                errors.extend(value_errors);
            }
        }
        Err(e) => {
            errors.push(ValidationError::new(
                path,
                format!("Failed to access map: {}", e),
            ));
        }
    }

    errors
}

/// Validate an enum value
fn validate_enum<'a>(value: Peek<'a>, path: &str) -> ValidationResult {
    let mut errors = Vec::new();

    let enum_result = value.into_enum();

    match enum_result {
        Ok(enum_value) => {
            // Get active variant and validate its fields
            let variant = enum_value.active_variant();

            for (index, field) in variant.data.fields.iter().enumerate() {
                if let Some(field_value) = enum_value.field(index) {
                    let field_path = build_path(path, field.name);
                    let field_errors = validate(field_value, field.shape(), &field_path);
                    errors.extend(field_errors);
                }
            }
        }
        Err(e) => {
            errors.push(ValidationError::new(
                path,
                format!("Failed to access enum: {}", e),
            ));
        }
    }

    errors
}
