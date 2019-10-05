use crate::ast::{ Identifier, Parameters, FunctionBlock };

pub struct Constructor {
    pub identifier : Identifier,
    pub parameters : Parameters
}

impl Constructor {
    pub fn new(identifier : Identifier, parameters : Parameters) -> Self {
        Self {
            identifier,
            parameters
        }
    }
}
