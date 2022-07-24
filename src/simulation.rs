use std::collections::HashMap;

use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    pub value: i32,
    spawner: Point,
}

impl Variable {
    pub fn reset(&mut self) {
        self.value = 0;
    }
}

pub struct Simulation {
    variables: HashMap<Point, Variable>,
    program: Program,
}

impl Simulation {
    pub fn find_variable(&self, position: Point) -> Option<&Variable> {
        self.variables.get(&position)
    }
}

pub enum SimulationStateChange {
    Move {
        variable_point: Point,
        new_position: Point,
    },
    Merge {
        variable_points: Vec<Point>,
        new_position: Point,
    },
    Spawn {
        spawner: Point,
    },
    Kill {
        variable_point: Point,
    },
    DoNothing,
}

impl Simulation {
    pub fn new(program: Program) -> Self {
        // Spawn a variable at each entrypoint
        let variables = program
            .entrypoints
            .iter()
            .map(|&point| {
                (
                    point,
                    Variable {
                        value: 0,
                        spawner: point,
                    },
                )
            })
            .collect();

        Self { variables, program }
    }

    pub fn print_map(&self) {
        println!();
        for y in self.program.bounds.min_y..=self.program.bounds.max_y {
            for x in self.program.bounds.min_x..=self.program.bounds.max_x {
                // Check if a variable is at this point
                if let Some(_) = self.find_variable(Point { x, y }) {
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
        // List to queue all changes this update
        let mut changes: Vec<SimulationStateChange> = Vec::new();

        // Move variables to the next location that their place points to
        self.variables.iter().for_each(|(point, variable)| {
            // If the variable is at the exit, move it back to the start
            if *point == self.program.exit {
                changes.push(SimulationStateChange::Kill {
                    variable_point: *point,
                });
            }

            let next_position = self
                .program
                .file
                .get(&point)
                .unwrap()
                .next
                .expect("Couldn't figure out where to go next");

            // Check the next position
            let next_place = self.program.file.get(&next_position).unwrap();

            // If there are multiple ways to get to this place, make sure other
            // each other position has a variable in it
            if next_place.prev.len() > 1 {
                // See if there's a variable in each position that leads into
                // this one
                if next_place
                    .prev
                    .iter()
                    .all(|point| self.find_variable(point.unwrap()).is_some())
                {
                    // Collect all of the variables that will be merging
                    let variables = next_place
                        .prev
                        .iter()
                        .map(|point| {
                            let point = point.unwrap();

                            let variable = self.find_variable(point).unwrap();

                            // Spawn new variables in their spawner's locations
                            changes.push(SimulationStateChange::Spawn {
                                spawner: variable.spawner,
                            });

                            point
                        })
                        .collect();

                    // Add the merge to the change list
                    changes.push(SimulationStateChange::Merge {
                        variable_points: variables,
                        new_position: next_position,
                    });
                } else {
                    // If not, the variable should stay put
                    changes.push(SimulationStateChange::DoNothing);
                }
            } else {
                // If there is only one way to get to this place, move the variable
                changes.push(SimulationStateChange::Move {
                    variable_point: *point,
                    new_position: next_position,
                });
            }
        });

        // Iterate over everything in the changes list
        for change in changes {
            match change {
                SimulationStateChange::Move {
                    variable_point,
                    new_position,
                } => {
                    // Add the variable to the new position
                    self.variables
                        .insert(new_position, *self.variables.get(&variable_point).unwrap());

                    // Remove the variable from the old position
                    self.variables.remove(&variable_point);
                }
                SimulationStateChange::Merge {
                    variable_points: variables,
                    new_position,
                } => {
                    // Merge the variables into a new variable, and complete the
                    // operation at this location
                    let values = variables
                        .iter()
                        .map(|point| self.variables.get(point).unwrap().value)
                        .collect::<Vec<i32>>();
                }
                SimulationStateChange::Spawn { spawner } => {
                    // Spawn a new variable at the spawner
                    let new_variable = Variable {
                        value: 0,
                        spawner: spawner,
                    };

                    self.variables.insert(spawner, new_variable);
                }
                SimulationStateChange::DoNothing => {}
                SimulationStateChange::Kill { variable_point } => {
                    // Kill the variable
                    self.variables.remove(&variable_point);
                }
            }
        }
    }
}
