use std::{collections::HashMap, fs};

fn main() {
    // Load the file in tests/hello_world
    let contents = fs::read_to_string("tests/hello_world").unwrap();

    println!("{}", contents);

    // Parse the file
    let mut map = parse(&contents);

    let program = Program::new(map);

    // Lex the file
    let tokens = lexer(&program);
}

type Point = (usize, usize);
type FileMap = HashMap<Point, char>;

struct Program {
    file: FileMap,
}

impl Program {
    fn new(file: FileMap) -> Program {
        Program { file }
    }
}

/// Parse the file into a map of points to characters
fn parse(file: &String) -> FileMap {
    // Make a hashmap of every character
    let mut map: FileMap = HashMap::new();

    // Iterate over the file
    file.lines().enumerate().for_each(|(i, line)| {
        // Iterate over the line
        line.chars().enumerate().for_each(|(j, c)| {
            // Add the character to the hashmap
            map.insert((i, j), c);
        });
    });

    map
}

/// Lex the file and extract tokens
fn lexer(program: &Program) {
    // Find the entrypoint
    let entrypoint_pos = program
        .file
        .iter()
        .find(|(_, c)| **c == Syntax::symbol(Syntax::Entrypoint))
        .unwrap().0;

    println!("{:?}", entrypoint_pos);
}

enum Syntax {
    Entrypoint,
}

impl Syntax {
    fn symbol(character: Syntax) -> char {
        match character {
            Syntax::Entrypoint => '1',
        }
    }
}
