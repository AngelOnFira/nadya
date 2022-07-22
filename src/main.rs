use std::fs;

fn main() {
    // Load the file in tests/hello_world
    let contents = fs::read_to_string("tests/hello_world").unwrap();

    println!("{}", contents);
}
