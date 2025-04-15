# facet-validate

Multi-field validation using Facet's reflection capabilities.

## Idea

Unlike traditional validation libraries that require extra traits and derive macros, facet-validate uses the `SHAPE` information provided by the `Facet` trait
("the last derive you'll ever need") which tells us what fields are expected [i.e. what to validate]
and their types.

## Features

- :white_check_mark: Validates JSON against any type implementing `Facet`
- :scroll: Collects all validation errors instead of failing at the first one (like `serde_valid`'s
  Validate trait)
- :mag: Path-aware error reporting for easy debugging (like `serde_path_to_error`)
- :tada: No additional derive macros needed beyond `Facet`

## Example

```rust
use facet::Facet;
use facet_validate::validate_json;

#[derive(Facet)]
struct User {
    name: String,
    age: u32,
    address: Address,
}

#[derive(Facet)]
struct Address {
    street: String,
    city: String,
}

fn main() {
    let json = r#"{
        "name": "Alice",
        "age": "thirty",
        "address": {
            "street": "123 Main St",
            "city": 42
        }
    }"#;

    let result = validate_json::<User>(json);

    // Shows all validation errors with paths
    for error in &result {
        println!("{}: {}", error.path, error.message);
    }
    // Output:
    // age: Type mismatch: expected u32, got string
    // address.city: Type mismatch: expected String, got number
}
```
