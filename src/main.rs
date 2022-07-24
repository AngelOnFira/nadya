use std::fs;

mod lexer;
mod parser;
mod place;
mod program;
mod simulation;
mod syntax;

use crate::lexer::lexer;

mod prelude {
    pub use crate::{lexer::*, parser::*, place::*, program::*, simulation::*, syntax::*};
}

use prelude::*;

fn main() {
    // Load the file in tests/hello_world
    let contents = fs::read_to_string("tests/hello_world").unwrap();

    println!("{}", contents);

    // Parse the file
    let mut program = parse(&contents);

    // Lex the file
    lexer(&mut program);

    // Run the simulation
    let mut simulation = Simulation::new(program);

    // Simulate a few times
    for _ in 0..10 {
        simulation.simulate();
        simulation.print_map();
    }
}
