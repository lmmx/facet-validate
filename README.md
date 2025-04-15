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

The `main.rs` module will run the following demo:

```rust
use facet::Facet;
use facet_validate::validate_json;

#[derive(Facet)]
struct FooBar {
    foo: u64,
    bar: String,
}

fn main() {
    // Using a simple, valid JSON string
    let json = r#"{"foo": 42, "bar": "Hello, World!"}"#;

    let result = validate_json::<FooBar>(json);

    if result.is_empty() {
        println!("Validation successful!");
    } else {
        println!("Validation errors:");
        for error in &result {
            println!("{}: {}", error.path, error.message);
        }
    }

    // Let's also try an invalid JSON to see error handling
    let invalid_json = r#"{"foo": 42, "bar": 42}"#;
    
    let result = validate_json::<FooBar>(invalid_json);
    
    println!("\nValidation of invalid JSON:");
    for error in &result {
        println!("{}: {}", error.path, error.message);
    }
}
```

- The first of these will say "Validation successful!"
- The second of these will say "Validation of invalid JSON:" (but no explanation yet)
