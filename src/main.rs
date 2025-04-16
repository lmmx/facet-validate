use facet::Facet;
use facet_validate::validate_json;

#[derive(Facet)]
struct FooBar {
    foo: u64,
    bar: String,
    baz: Vec<u64>,
}

fn main() {
//     // Using a simple, valid JSON string
//     let json = r#"{"foo": 42, "bar": "Hello, World!", "baz": [1,2,3]}"#;
// 
//     let result = validate_json::<FooBar>(json);
// 
//     if result.is_empty() {
//         println!("Validation successful!");
//     } else {
//         println!("Validation errors:");
//         for error in &result {
//             println!("{}: {}", error.path, error.message);
//         }
//     }
// 
    // Let's also try an invalid JSON to see error handling
    let invalid_json = r#"{"foo": 42, "bar": 42}"#;
    
    let result = validate_json::<FooBar>(invalid_json);
    
    // println!("\nValidation of invalid String field JSON by JSON reader:");
    if result.is_empty() {
        println!("No validation errors detected. The JSON validation didn't work!");
    } else {
        for error in &result {
            println!("{}", error.message);
        }
    }

    // // Let's also try an invalid JSON to see list error handling
    // let invalid_list_json = r#"{"foo": 42, "bar": "Hello world", "baz": ["oops"]}"#;
    // 
    // let result = validate_json::<FooBar>(invalid_list_json);
    // 
    // println!("\nValidation of invalid Vec<u64> field JSON by JSON reader:");
    // if result.is_empty() {
    //     println!("No validation errors detected. The JSON validation didn't work!");
    // } else {
    //     for error in &result {
    //         println!("{}: {}", error.path, error.message);
    //     }
    // }
}
