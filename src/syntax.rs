#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Syntax {
    Entrypoint,
    Exit,
    VerticalConnector,
    HorizontalConnector,
    IntersectingConnector,
    Floor,
    // Operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Max,
    Min,
    GreaterThan,
    LessThan,
    Equal,
}

impl Syntax {
    pub fn symbol(character: Syntax) -> char {
        match character {
            Syntax::Entrypoint => '1',
            Syntax::Exit => '9',
            Syntax::VerticalConnector => '|',
            Syntax::HorizontalConnector => '_',
            Syntax::IntersectingConnector => '#',
            Syntax::Floor => '.',
            // Operations
            Syntax::Add => '+',
            Syntax::Subtract => '-',
            Syntax::Multiply => '*',
            Syntax::Divide => '/',
            Syntax::Modulo => '%',
            Syntax::Max => '^',
            Syntax::Min => 'v',
            Syntax::GreaterThan => '>',
            Syntax::LessThan => '<',
            Syntax::Equal => '=',
        }
    }

    // TODO: Make a function that verifies that the syntax is used with the
    // right number of paramaters

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
            '_' => Syntax::HorizontalConnector,
            '.' => Syntax::Floor,
            '#' => Syntax::IntersectingConnector,
            // Operations
            '+' => Syntax::Add,
            '-' => Syntax::Subtract,
            '*' => Syntax::Multiply,
            '/' => Syntax::Divide,
            '%' => Syntax::Modulo,
            '^' => Syntax::Max,
            'v' => Syntax::Min,
            '>' => Syntax::GreaterThan,
            '<' => Syntax::LessThan,
            '=' => Syntax::Equal,

            _ => panic!("Unknown character"),
        }
    }
}
