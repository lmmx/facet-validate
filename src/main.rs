use facet::Facet;
use facet_validate::validate_json;

#[derive(Facet)]
struct Angery {
    #[facet(rename = "@@@")]
    swearing: String,
}

fn main() {
    let fieldname = "@@@";
    let fieldname_fmt = format!("{}", fieldname);
    println!("Trying: {}", fieldname_fmt);
    let json = r#"{"@@@": "please and thank you"}"#;
    
    let result = validate_json::<Angery>(json);
    
    if result.is_empty() {
        println!("No validation errors detected. Success!");
    } else {
        for error in &result {
            println!("{}", error.message);
        }
    }
}
