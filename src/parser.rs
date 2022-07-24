use std::collections::HashMap;

use crate::prelude::*;

/// Parse the file into a map of points to characters
pub fn parse(file: &String) -> FileMap {
    // Make a hashmap of every character
    let mut map: FileMap = HashMap::new();

    // Iterate over the file
    file.lines().enumerate().for_each(|(i, line)| {
        // Iterate over the line
        line.chars().enumerate().for_each(|(j, c)| {
            // Add the character to the hashmap
            map.insert(
                Point {
                    x: j as i32,
                    y: i as i32,
                },
                Place::new(c.into()),
            );
        });
    });

    map
}
