use crate::ast::{Identifier, FunctionBlock};
use std::fmt;

pub struct DestructorImpl {
    pub identifier : Identifier
}

impl DestructorImpl {
    pub fn new(identifier : Identifier) -> Self {
        Self {
            identifier
        }
    }
}

impl fmt::Display for DestructorImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{class_id}::~{class_id}() {{ {class_id}_destroy(*this); }}", class_id = self.identifier)
    }
}
