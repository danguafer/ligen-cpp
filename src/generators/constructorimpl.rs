use crate::ast::{Identifier, Expression, ConstructorImpl, FunctionCall, FunctionBlock, ConstructorInitializer};
use crate::generators::{ConstructorGenerator, ArgsGenerator};

pub struct ConstructorImplGenerator {}

impl ConstructorImplGenerator {
    pub fn generate(method : &ligen_core::Method, sized_integer : bool) -> ConstructorImpl {
        let function_statements = Vec::new();

        let args = ArgsGenerator::generate(&method.inputs);
        let function_call = FunctionCall::new(Identifier::new(&method.get_extern_name()), args);
        let initializer = ConstructorInitializer::new(Identifier::new(&method.owner.identifier.name), Expression::FunctionCall(function_call));
        let initializers = vec![initializer];

        let function_block = FunctionBlock::new(function_statements);
        ConstructorImpl::new(ConstructorGenerator::generate(&method, sized_integer), initializers, function_block)
    }
}
