use std::collections::HashSet;

use crate::prelude::*;

/// Lex the file and extract tokens
pub fn lexer(program: &mut Program) {
    // Find the entrypoint
    let entrypoint_place = program
        .file
        .iter()
        .find(|(_, c)| c.syntax == Syntax::Exit)
        .unwrap()
        .clone()
        .0
        .clone();

    println!("{:?}", entrypoint_place);

    // Find every command connected to the entrypoint
    let mut commands_queue = vec![entrypoint_place];

    // Track the locations we've already visited
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(entrypoint_place);

    while let Some(this_point) = commands_queue.pop() {
        // Look in the four directions around this position
        for direction in [(0i32, -1i32), (0, 1), (-1, 0), (1, 0)] {
            // Get the position in the direction
            let new_pos_point = Point {
                x: this_point.x + direction.0,
                y: this_point.y + direction.1,
            };

            // Get the character at the new position
            let new_pos_syntax = program.file.get(&new_pos_point).unwrap().syntax.clone();

            match new_pos_syntax {
                Syntax::Entrypoint
                | Syntax::VerticalConnector
                | Syntax::HorizontalConnector
                | Syntax::IntersectingConnector => {
                    // Add the new position to the queue
                    if !visited.contains(&new_pos_point) {
                        // Update the next position of the found location
                        program.file.get_mut(&new_pos_point).unwrap().next = Some(new_pos_point);

                        // Update the previous position list of this position
                        program
                            .file
                            .get_mut(&this_point)
                            .unwrap()
                            .prev
                            .push(Some(new_pos_point));

                        commands_queue.push(new_pos_point);
                        visited.insert(new_pos_point);
                    }
                }
                _ => {}
            }
        }
    }
}
