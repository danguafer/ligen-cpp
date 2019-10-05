use std::fmt;

#[derive(PartialEq)]
pub enum Modifier {
    Move,
    Reference,
    Pointer,
    None
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Modifier::Move => write!(f, "&&"),
            Modifier::Reference => write!(f, "&"),
            Modifier::Pointer => write!(f, "*"),
            Modifier::None => write!(f, "")
        }
    }
}
