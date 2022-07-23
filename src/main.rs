use std::{
    collections::{HashMap, HashSet},
    fs,
};

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point(i32, i32);

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
            map.insert(Point(i as i32, j as i32), c.into());
        });
    });

    map
}

/// Lex the file and extract tokens
fn lexer(program: &Program) {
    // Find the entrypoint
    let entrypoint_pos = *program
        .file
        .iter()
        .find(|(_, c)| **c == Syntax::symbol(Syntax::Exit))
        .unwrap()
        .0;

    println!("{:?}", entrypoint_pos);

    // Find every command connected to the entrypoint
    let mut commands_queue = vec![entrypoint_pos];

    // Track the locations we've already visited
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(entrypoint_pos);

    while let Some(pos) = commands_queue.pop() {
        // Get the character at the position
        let c = program.file.get(&pos).unwrap();

        // Look in the four directions around this position
        for direction in [(0i32, -1i32), (0, 1), (-1, 0), (1, 0)] {
            // Get the position in the direction
            let new_pos = Point(pos.0 + direction.0, pos.1 + direction.1);

            // Get the character at the new position
            let new_c = program.file.get(&new_pos).unwrap();

            // If the character is a command, add it to the queue
            match new_c.try_into() {
                Ok(command) => {
                    match command {
                        // If it's a connector, add that to the path
                        Syntax::VerticalConnector | Syntax::HorizontalConnector => {
                            if !visited.contains(&new_pos) {
                                commands_queue.push(new_pos);
                                visited.insert(new_pos);
                            }
                        }
                        // If we found an entrypoint, print it out
                        Syntax::Entrypoint => {
                            if !visited.contains(&new_pos) {
                                commands_queue.push(new_pos);
                                visited.insert(new_pos);
                            }
                            println!("Found entry at {:?}", new_pos);
                        }
                        _ => {}
                    }
                }
                Err(_) => todo!(),
            }
        }
    }
}

enum Syntax {
    Entrypoint,
    Exit,
    VerticalConnector,
    HorizontalConnector,
    IntersectingConnector,
    Floor,
}

impl Syntax {
    fn symbol(character: Syntax) -> char {
        match character {
            Syntax::Entrypoint => '1',
            Syntax::Exit => '9',
            Syntax::VerticalConnector => '|',
            Syntax::HorizontalConnector => '-',
            Syntax::IntersectingConnector => '+',
            Syntax::Floor => '.',
        }
    }
}

/// Convert a character to a syntax
impl From<&char> for Syntax {
    fn from(character: &char) -> Syntax {
        match character {
            '1' => Syntax::Entrypoint,
            '9' => Syntax::Exit,
            '|' => Syntax::VerticalConnector,
            '-' => Syntax::HorizontalConnector,
            '.' => Syntax::Floor,
            '+' => Syntax::IntersectingConnector,
            _ => panic!("Unknown character"),
        }
    }
}
