use std::{
    collections::{HashMap, HashSet},
    fs,
};

mod lexer;
mod parser;
mod program;
mod syntax;

use crate::lexer::lexer;

mod prelude {
    pub use crate::{lexer::*, parser::*, program::*, syntax::*, Place, Point};
}

use prelude::*;

fn main() {
    // Load the file in tests/hello_world
    let contents = fs::read_to_string("tests/hello_world").unwrap();

    println!("{}", contents);

    // Parse the file
    let mut map = parse(&contents);

    let mut program = Program::new(map);

    // Lex the file
    lexer(&mut program);

    for (_, place) in program.file.iter() {
        match place.syntax {
            Syntax::Floor => (),
            _ => {
                dbg!(place);
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Place {
    next: Option<Point>,
    prev: Vec<Option<Point>>,
    syntax: Syntax,
}

impl Place {
    fn new(syntax: Syntax) -> Self {
        Self {
            next: None,
            prev: Vec::new(),
            syntax,
        }
    }
}
