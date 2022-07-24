#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Syntax {
    Entrypoint,
    Exit,
    VerticalConnector,
    HorizontalConnector,
    IntersectingConnector,
    Floor,
}

impl Syntax {
    pub fn symbol(character: Syntax) -> char {
        match character {
            Syntax::Entrypoint => '1',
            Syntax::Exit => '9',
            Syntax::VerticalConnector => '|',
            Syntax::HorizontalConnector => '-',
            Syntax::IntersectingConnector => '+',
            Syntax::Floor => '.',
        }
    }

    pub fn get_symbol(self) -> char {
        Self::symbol(self)
    }
}

/// Convert a character to a syntax
impl From<char> for Syntax {
    fn from(character: char) -> Syntax {
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
