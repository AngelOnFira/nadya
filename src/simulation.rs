use crate::prelude::*;

pub struct Variable {
    pub value: i32,
    pub position: Point,
    spawner: Point,
}

impl Variable {
    pub fn reset(&mut self) {
        self.value = 0;
        self.position = self.spawner;
    }
}

pub struct Simulation {
    variables: Vec<Variable>,
    program: Program,
}

impl Simulation {
    pub fn new(program: Program) -> Self {
        // Spawn a variable at each entrypoint
        let variables = program
            .entrypoints
            .iter()
            .map(|&point| Variable {
                value: 0,
                position: point,
                spawner: point,
            })
            .collect();

        Self { variables, program }
    }

    pub fn print_map(&self) {
        println!();
        for y in self.program.bounds.min_y..=self.program.bounds.max_y {
            for x in self.program.bounds.min_x..=self.program.bounds.max_x {
                // Check if a variable is at this point
                if let Some(_) = self.variables.iter().find(|v| v.position == Point { x, y }) {
                    // If so, represent it with an O
                    print!("O");
                } else {
                    let point = Point { x, y };
                    let place = self.program.file.get(&point).unwrap();

                    // If it's a floor, use a space instead of a period
                    let symbol = match place.syntax {
                        Syntax::Floor => ' ',
                        _ => place.syntax.get_symbol(),
                    };

                    print!("{symbol}");
                }
            }
            println!();
        }
    }

    pub fn simulate(&mut self) {
        // Move variables to the next location that their place points to
        self.variables.iter_mut().for_each(|variable| {
            // If the variable is at the exit, move it back to the start
            if variable.position == self.program.exit {
                variable.reset();
            }

            variable.position = self
                .program
                .file
                .get(&variable.position)
                .unwrap()
                .next
                .expect("Couldn't figure out where to go next");
        });
    }
}
