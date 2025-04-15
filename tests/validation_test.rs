use facet::Facet;
use facet_validate::validate_json;

#[derive(Debug, PartialEq, Eq, Facet)]
struct FooBar {
    foo: u64,
    bar: String,
}

#[derive(Debug, PartialEq, Facet)]
struct User {
    name: String,
    age: u32,
    address: Address,
}

#[derive(Debug, PartialEq, Facet)]
struct Address {
    street: String,
    city: String,
}

#[test]
fn test_valid_json() {
    let valid_json = r#"{"foo": 42, "bar": "Hello, World!"}"#;
    let result = validate_json::<FooBar>(valid_json);
    assert!(result.is_empty(), "Expected no validation errors");
}

#[test]
fn test_invalid_simple_json() {
    let invalid_json = r#"{"foo": "not a number", "bar": 42}"#;
    let result = validate_json::<FooBar>(invalid_json);
    
    assert_eq!(result.len(), 2, "Expected exactly 2 validation errors");
    
    // Check paths are correct
    let paths: Vec<_> = result.iter().map(|e| &e.path).collect();
    assert!(paths.contains(&&"foo".to_string()));
    assert!(paths.contains(&&"bar".to_string()));
}

#[test]
fn test_nested_validation() {
    let invalid_json = r#"{
        "name": "Alice",
        "age": "thirty",
        "address": {
            "street": "123 Main St",
            "city": 42
        }
    }"#;
    
    let result = validate_json::<User>(invalid_json);
    
    assert_eq!(result.len(), 2, "Expected exactly 2 validation errors");
    
    // Check paths are correct
    let paths: Vec<_> = result.iter().map(|e| &e.path).collect();
    assert!(paths.contains(&&"age".to_string()));
    assert!(paths.contains(&&"address.city".to_string()));
}

#[test]
fn test_missing_fields() {
    let incomplete_json = r#"{"foo": 42}"#;
    let result = validate_json::<FooBar>(incomplete_json);
    
    assert_eq!(result.len(), 1, "Expected exactly 1 validation error");
    assert_eq!(result[0].path, "bar");
}