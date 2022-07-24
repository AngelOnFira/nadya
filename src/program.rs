use std::collections::HashMap;

use crate::prelude::*;

pub type FileMap = HashMap<Point, Place>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    pub file: FileMap,
}

impl Program {
    pub fn new(file: FileMap) -> Program {
        Program { file }
    }
}
