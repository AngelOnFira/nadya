use std::collections::HashSet;

use crate::prelude::*;

/// Lex the file and extract tokens
pub fn lexer(program: &mut Program) {
    // Find the entrypoint
    let exit_point = program
        .file
        .iter()
        .find(|(_, c)| c.syntax == Syntax::Exit)
        .unwrap()
        .clone()
        .0
        .clone();

    program.exit = exit_point;

    // Find every command connected to the entrypoint
    let mut commands_queue = vec![exit_point];

    // Track the locations we've already visited
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(exit_point);

    while let Some(this_point) = commands_queue.pop() {
        // Look in the four directions around this position
        for direction in [(0i32, -1i32), (0, 1), (-1, 0), (1, 0)] {
            // Get the position in the direction
            let new_pos_point = Point {
                x: this_point.x + direction.0,
                y: this_point.y + direction.1,
            };

            // Get the character at the new position
            let new_pos_syntax = program.file.get(&new_pos_point).unwrap().syntax;

            match new_pos_syntax {
                Syntax::Entrypoint
                | Syntax::VerticalConnector
                | Syntax::HorizontalConnector
                | Syntax::IntersectingConnector
                | Syntax::Add
                | Syntax::Subtract
                | Syntax::Multiply
                | Syntax::Divide
                | Syntax::Modulo
                | Syntax::Max
                | Syntax::Min
                | Syntax::GreaterThan
                | Syntax::LessThan
                | Syntax::Equal => {
                    // Add the new position to the queue
                    if !visited.contains(&new_pos_point) {
                        // Update the next position of the found location
                        program.file.get_mut(&new_pos_point).unwrap().next = Some(this_point);

                        // Update the previous position list of this position
                        program
                            .file
                            .get_mut(&this_point)
                            .unwrap()
                            .prev
                            .push(Some(new_pos_point));

                        commands_queue.push(new_pos_point);
                        visited.insert(new_pos_point);

                        match new_pos_syntax {
                            Syntax::Entrypoint => {
                                // Update the entrypoint
                                program.entrypoints.push(new_pos_point);
                            }
                            Syntax::Exit => {
                                // Update the exitpoint
                                program.exit = new_pos_point;
                            }
                            _ => (),
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
