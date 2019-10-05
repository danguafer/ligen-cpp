use crate::ast::{Identifier, Modifier};
use std::fmt;

pub struct Type {
    pub constness : bool,
    pub identifier : Identifier,
    pub modifier : Modifier
}

impl Type {
    pub fn new(constness : bool, identifier : Identifier, modifier : Modifier) -> Self {
        Self {
            constness,
            identifier,
            modifier
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.constness {
            write!(f, "const ")?;
        }
        write!(f, "{}{}", self.identifier, self.modifier)
    }
}
