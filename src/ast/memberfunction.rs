use crate::ast::{Type, Identifier, Parameters};

pub struct MemberFunction {
    pub return_type : Type,
    pub identifier : Identifier,
    pub parameters : Parameters,
    pub constness : bool
}

impl MemberFunction {
    pub fn new(return_type : Type, identifier : Identifier, parameters : Parameters, constness : bool) -> Self {
        Self {
            return_type,
            identifier,
            parameters,
            constness
        }
    }
}
