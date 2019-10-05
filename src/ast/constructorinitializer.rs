use crate::ast::{Identifier, Expression};

pub struct ConstructorInitializer {
    pub identifier : Identifier,
    pub expression : Expression
}

impl ConstructorInitializer {
    pub fn new(identifier : Identifier, expression : Expression) -> Self {
        Self {
            identifier,
            expression
        }
    }
}

use std::fmt;
impl fmt::Display for ConstructorInitializer {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self.identifier, self.expression)
    }
}
