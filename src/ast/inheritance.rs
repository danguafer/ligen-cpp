use crate::ast::{Identifier, Access};

pub struct Inheritance {
    pub access : Access,
    pub identifier : Identifier
}

impl Inheritance {
    pub fn new(access : Access, identifier : Identifier) -> Self {
        Self {
            access,
            identifier
        }
    }
}

use std::fmt;
impl fmt::Display for Inheritance {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.access, self.identifier)
    }
}
