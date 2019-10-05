use crate::ast::{Identifier, Inheritance, ClassStatement};
use std::fmt;

pub struct ClassDecl {
    identifier : Identifier,
    inheritances : Vec<Inheritance>,
    statements : Vec<ClassStatement>
}

impl ClassDecl {
    pub fn new(identifier : Identifier, inheritances : Vec<Inheritance>, statements : Vec<ClassStatement>) -> Self {
        Self {
            identifier,
            inheritances,
            statements
        }
    }
}

impl fmt::Display for ClassDecl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "class {}", self.identifier)?;
        for (i, inheritance) in self.inheritances.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            } else {
                write!(f, " : ")?;
            }
            write!(f, "{}", inheritance)?
        }
        write!(f, " {{")?;
        for statement in &self.statements {
            write!(f, "\n\t{}", statement)?;
        }
        write!(f, "\n}};")
    }
}
