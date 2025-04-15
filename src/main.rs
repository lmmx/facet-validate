use facet::Facet;
use facet_validate::validate_json;

#[derive(Facet)]
struct FooBar {
    foo: u64,
    bar: String,
}

fn main() {
    // Using a simple, compact JSON string
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
