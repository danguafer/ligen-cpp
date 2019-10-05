use crate::ast::{Constructor, ConstructorInitializer, FunctionBlock};
use std::fmt;

pub struct ConstructorImpl {
    constructor : Constructor,
    initializers : Vec<ConstructorInitializer>,
    function_block : FunctionBlock
}

impl ConstructorImpl {
    pub fn new(constructor : Constructor, initializers : Vec<ConstructorInitializer>, function_block : FunctionBlock) -> Self {
        Self {
            constructor,
            initializers,
            function_block
        }
    }
}

impl fmt::Display for ConstructorImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{class_id}::{class_id}({parameters})", class_id = self.constructor.identifier, parameters = self.constructor.parameters)?;
        for (i, initializer) in self.initializers.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            else { write!(f, " : ")?; }
            write!(f, "{}", initializer)?;
        }
        write!(f, " {}", self.function_block)
    }
}
