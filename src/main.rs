use facet::Facet;

#[derive(Debug, PartialEq, Eq, Facet)]
struct FooBar {
    foo: u64,
    bar: String,
}

fn main() {
    println!("Hello, world!");
}
