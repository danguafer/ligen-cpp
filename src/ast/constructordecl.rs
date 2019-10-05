use crate::ast::{Constructor};
use std::fmt;

pub struct ConstructorDecl {
    constructor : Constructor
}

impl ConstructorDecl {
    pub fn new(constructor : Constructor) -> Self {
        Self {
            constructor
        }
    }
}

impl fmt::Display for ConstructorDecl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self.constructor.identifier, self.constructor.parameters)
    }
}
