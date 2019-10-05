use crate::ast::{Identifier};
use std::fmt;

pub struct DestructorDecl {
    pub identifier : Identifier
}

impl DestructorDecl {
    pub fn new(identifier : Identifier) -> Self {
        Self {
            identifier
        }
    }
}


impl fmt::Display for DestructorDecl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "~{}()", self.identifier)
    }
}
